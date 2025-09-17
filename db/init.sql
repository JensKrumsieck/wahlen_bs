PRAGMA foreign_keys = ON;
PRAGMA encoding="UTF-8";

CREATE TABLE party (
        id TEXT NOT NULL PRIMARY KEY,
        name TEXT NOT NULL
);

CREATE TABLE election (
        id INTEGER PRIMARY KEY, 
        date INTEGER NOT NULL, 
        name TEXT NOT NULL
);

CREATE TABLE region (
        id INTEGER PRIMARY KEY, 
        name TEXT NOT NULL
);

INSERT INTO region (id, name) VALUES 
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
        (322, "Nördliche Schunter-/Okeraue"),
        (330, "Nordstadt-Schunteraue");

CREATE TABLE vote (
        id INTEGER PRIMARY KEY,
        election_id INTEGER NOT NULL,
        region_id INTEGER NOT NULL,
        party_id TEXT NOT NULL,
        votes INTEGER NOT NULL,
        primary_vote BOOLEAN NOT NULL,
        FOREIGN KEY (election_id) REFERENCES election (id),
        FOREIGN KEY (region_id) REFERENCES region (id),        
        FOREIGN KEY (party_id) REFERENCES party (id)
);

CREATE TABLE turnout (
        election_id INTEGER NOT NULL,
        region_id INTEGER NOT NULL,
        eligible INTEGER NOT NULL,
        voted INTEGER NOT NULL,
        primary_vote BOOLEAN NOT NULL,
        PRIMARY KEY (election_id, region_id, primary_vote),
        FOREIGN KEY (election_id) REFERENCES election (id),
        FOREIGN KEY (region_id) REFERENCES region (id)
);