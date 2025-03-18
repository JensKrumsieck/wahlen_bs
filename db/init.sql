PRAGMA foreign_keys = ON;

CREATE TABLE party (
        id INTEGER PRIMARY KEY,
        name TEXT,
        abbreviation TEXT
);

CREATE TABLE election (
    id INTEGER PRIMARY KEY, 
    date TEXT, 
    name TEXT
);

CREATE TABLE region (
    id INTEGER PRIMARY KEY, 
    num INTEGER, name TEXT
);

INSERT INTO region (num, name) VALUES 
        (111, "Hondelage-Volkmarode"),
        (112, "Wabe-Schunter-Beberbach"),
        (120, "Östliches Ringgebiet"),
        (130, "Mitte"),
        (211, "Braunschweig-Süd"),
        (212, "Südstadt-Rautheim-Mascherode"),
        (221, "Weststadt"),
        (222, "Südwest"),
        (310, "Westliches Ringgebiet"),
        (321, "Lehndorf-Watenbüttel"),
        (322, "Nördlicher Schunter-/Okeraue"),
        (330, "Nordstadt-Schunteraue");

CREATE TABLE vote (
        id INTEGER PRIMARY KEY,
        election_id INTEGER,
        region_id INTEGER,
        party_id INTEGER,
        votes INTEGER,
        primary_vote BOOLEAN,
        FOREIGN KEY (election_id) REFERENCES election (id),
        FOREIGN KEY (region_id) REFERENCES region (id),        
        FOREIGN KEY (party_id) REFERENCES party (id)
);

CREATE TABLE turnout (
        election_id INTEGER,
        region_id INTEGER,
        eligible INTEGER,
        voted INTEGER,
        primary_vote BOOLEAN,
        PRIMARY KEY (election_id, region_id, primary_vote),
        FOREIGN KEY (election_id) REFERENCES election (id),
        FOREIGN KEY (region_id) REFERENCES region (id)
);