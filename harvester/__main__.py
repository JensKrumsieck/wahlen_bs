from collections import defaultdict
import json
import requests
import sqlite3
import datetime
import pandas as pd
import os
import re
from bs4 import BeautifulSoup
from urllib.parse import urljoin
from dictionaries import dict_parties, dict_canonical_parties, dict_group_mapping, dict_election_name

try:
    os.remove("db/elections.db")
except:
    pass
db = sqlite3.connect("db/elections.db")
check = db.execute(
    "SELECT name FROM sqlite_master WHERE type='table' AND name='election'")
if not check.fetchone():
    with open("db/init.sql") as f:
        cursor = db.cursor()
        cursor.executescript(f.read())
        db.commit()

# votemanager (2019-2025)

city_id = "03101000"
base_url = "https://votemanager.kdo.de"
dates_url = f"{base_url}/{city_id}/api/termine.json"

dates = requests.get(dates_url).json()["termine"]

# no data before 2018, OB elections not relevant for this project except europawahl until 2000
dates = [date for date in dates
         if datetime.datetime.strptime(date["date"], "%d.%m.%Y").year >= 2018 and not "meister" in date["name"]
         or date["name"].startswith("Europawahl") and datetime.datetime.strptime(date["date"], "%d.%m.%Y").year >= 2000
         or datetime.datetime.strptime(date["date"], "%d.%m.%Y").year < 2011 and datetime.datetime.strptime(date["date"], "%d.%m.%Y").year >= 2000]

urls = [{"name": date["name"], "url": urljoin(
    base_url, date["url"]), "date": datetime.datetime.strptime(date["date"], "%d.%m.%Y").year} for date in dates]

open_data_url = "../daten/opendata/open_data.json"
open_data_url_alt = "../api/praesentation/open_data.json"

open_data = [
    {
        "name": "Bundestagswahl" if url["name"].startswith("Wahl") and url["date"] == 2009
        else "Landtagswahl" if url["name"].startswith("Wahl") and url["date"] == 2008
        else url["name"].split(" ")[0],
        "date": url["date"],
        "url": urljoin(url["url"], open_data_url) if url["name"] != "Landtagswahl"
        else urljoin(url["url"], open_data_url_alt)
    }
    for url in urls
]

for item in open_data:
    election_selector = item["url"]
    data = requests.get(election_selector).json()

    res = db.execute("INSERT INTO election (name, date) VALUES (?, ?)",
                     (item["name"], item["date"]))
    db.commit()
    election_id = res.lastrowid

    # insert parties and get id
    parties = data["dateifelder"][0]["parteien"]
    party_mapping = {}
    for party in parties:
        party_short = dict_parties.get(party["wert"], party["wert"])
        res = db.execute(
            "SELECT id FROM party WHERE abbreviation LIKE ?", (party_short,))
        row = res.fetchone()
        if not row:
            cursor = db.cursor()
            canonical = dict_canonical_parties.get(party_short, party["wert"])
            cursor.execute(
                "INSERT INTO party (abbreviation, name) VALUES (?, ?)", (party_short, canonical))
            db.commit()
            party_id = cursor.lastrowid
        else:
            party_id = row[0]
        party_column_id = re.match(r"^D(\d+)", party["feld"]).group(1)
        party_mapping[party_column_id] = party_id

    # open csv file
    file = [urljoin(election_selector, f"../../praesentation/{csv['url']}")
            if "Landtagswahl" in csv["url"]
            else urljoin(election_selector, csv["url"])
            for csv in data["csvs"]
            if "Stadtbezirk" in csv["ebene"]
            if not "meister" in csv["wahl"]][-1]

    df = pd.read_csv(file, sep=";", encoding="utf-8")
    df["gebiet-nr"] = df["gebiet-name"].astype(str).str[:3]
    if df.shape[0] > 13:
        # recalculate old districts
        df["gebiet-nr"] = df["gebiet-nr"].map(dict_group_mapping)
        df = df.groupby("gebiet-nr", as_index=False).sum()

    def add_votes(row, election_id, region_id, party_mapping, primary_vote, max_votes, valid_votes):
        # add turnout / weird values for Kommunalwahl
        cursor = db.cursor()
        cursor.execute("INSERT INTO turnout (election_id, region_id, eligible, voted, primary_vote) VALUES (?, ?, ?, ?, ?)",
                       (election_id, region_id, max_votes, valid_votes, primary_vote))

        # only secondary votes
        for column_id, party_id in party_mapping.items():
            if primary_vote:
                columnname = "F" + column_id
            else:
                columnname = "D" + column_id + \
                    "_summe_liste_kandidaten" if item["name"] == "Kommunalwahlen" else "D" + column_id
            votes = row[columnname]
            if votes and votes > 0:
                cursor.execute(
                    "INSERT INTO vote (election_id, region_id, party_id, votes, primary_vote) VALUES (?, ?, ?, ?, ?)", (election_id, region_id, party_id, votes, primary_vote))
        db.commit()

    # add votes to db
    for _, row in df.iterrows():
        res = db.execute("SELECT id FROM region WHERE num = ?",
                         (row["gebiet-nr"],))
        region_id = res.fetchone()[0]

        max_votes = row["A"]
        valid_votes = row["D"]
        add_votes(row, election_id, region_id, party_mapping,
                  False, max_votes, valid_votes)

        if item["name"] in ["Bundestagswahl", "Landtagswahl"]:
            max_votes = row["A"]
            valid_votes = row["F"]
            add_votes(row, election_id, region_id, party_mapping,
                      True, max_votes, valid_votes)
# end votemanager

# start before 2018
base_url = "https://www3.braunschweig.de/wahlen/ergebnis"
elections = [
    {"url": "bw17", "id": 372},
    {"url": "lw17", "id": 383},
    {"url": "bw13", "id": 312},
    {"url": "lw13", "id": 303}
]

for election_selector in elections:
    name = dict_election_name[election_selector["url"][:-2]]
    year = 2000 + int(election_selector["url"][-2:])
    cursor = db.cursor()
    cursor.execute("INSERT INTO election (name, date) VALUES (?, ?)",
                   (name, year))
    db.commit()
    election_id = cursor.lastrowid

    election_url = f"{base_url}_{election_selector['url']}/ajax.php?site=right/ergebnis&wahl={election_selector['id']}&anzeige=0&idx=0&typ=4&gID=1&gTyp=3&flip=1&mode=liste&hoch=0"
    districts = [83, 3, 4, 5, 69, 82, 71, 7, 8,
                 72, 76, 77, 78, 79, 80, 81, 73, 74, 75]
    votes = [1, 2]

    for vote in votes:
        rows = []
        for district in districts:
            url = election_url + f"&gebiet={district}&stimme={vote}"

            body = requests.get(url).text
            soup = BeautifulSoup(body, "html.parser")

            try:
                region = soup.find("div", {"class": "ergebnisTabKopf"}).find(
                    "table").find_all("tr")[2].find_all("td")[1].text.strip()
            except:
                region = soup.find("h3").text
                region = re.search(
                    r"\d{3} [\w-]*", str(region), re.MULTILINE).group(0)

            region_num = dict_group_mapping[region[:3]]

            # get turnout
            details = soup.find("div", {"class", "ergebniskopf"}).find_all(
                "div", class_=["span2", "span3"])
            max_voters = int(details[1].text.split(":")[
                1].strip().replace(".", ""))
            valid_votes = int(details[3].text.split(":")[
                1].strip().replace(".", ""))

            # get results
            results = soup.find(
                "table", {"class": "table table-striped table-bordered"}).find("tbody").find_all("tr")
            election_votes = {}
            skip_next = False

            for i in range(len(results)):
                if skip_next:
                    skip_next = False
                    continue

                result = results[i]
                party = result.find_all("td")[0].text.strip()
                if vote == 1:
                    # newer versions Candidate (Party)
                    try:
                        party = party.split("(")[1].split(")")[0]
                    # older versions Party\nCandidate
                    except:
                        result = results[i + 1]
                        skip_next = True
                party = dict_parties.get(party, party)

                item_votes = int(result.find_all(
                    "td")[1].text.strip().replace(".", ""))
                election_votes[party] = item_votes
            rows.append({"gebiet": region, "gebiet-nr": region_num, "primary_vote": vote ==
                        1, "max_voters": max_voters, "valid_votes": valid_votes, **election_votes})
        df = pd.DataFrame(rows)
        df = df.groupby("gebiet-nr", as_index=False).sum()

        # add to db
        for _, row in df.iterrows():
            res = db.execute("SELECT id FROM region WHERE num = ?",
                             (row["gebiet-nr"],))
            region_id = res.fetchone()[0]
            cursor = db.cursor()
            cursor.execute("INSERT INTO turnout (election_id, region_id, eligible, voted, primary_vote) VALUES (?, ?, ?, ?, ?)",
                           (election_id, region_id, row["max_voters"], row["valid_votes"], row["primary_vote"] != 0))
            for party, votes in row.items():
                if party in ["gebiet", "gebiet-nr", "primary_vote", "max_voters", "valid_votes"]:
                    continue
                cursor.execute(
                    "SELECT id FROM party WHERE abbreviation LIKE ?", (party,))
                pt = cursor.fetchone()
                if not pt:
                    cursor.execute("INSERT INTO party (abbreviation, name) VALUES (?, ?)",
                                   (party, dict_canonical_parties.get(party, party)))
                    party_id = cursor.lastrowid
                else:
                    party_id = pt[0]

                cursor.execute("INSERT INTO vote (election_id, region_id, party_id, votes, primary_vote) VALUES (?, ?, ?, ?, ?)",
                               (election_id, region_id, party_id, votes, row["primary_vote"] != 0))
            db.commit()
# end before 2018    
db.close()
