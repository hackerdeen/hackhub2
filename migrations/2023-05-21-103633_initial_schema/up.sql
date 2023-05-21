CREATE TABLE address
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    user      INTEGER NOT NULL,
    line_1    TEXT    NOT NULL,
    line_2    TEXT,
    line_3    TEXT,
    post_town TEXT    NOT NULL,
    postcode  TEXT    NOT NULL,
    country   TEXT    NOT NULL
);

CREATE TABLE person_data
(
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    user                  INTEGER NOT NULL,
    legal_name            INTEGER NOT NULL,
    service_address       INTEGER NOT NULL,
    residential_address   INTEGER,
    state                 TEXT    NOT NULL,
    submitted_time        TIMESTAMP,
    registrar_action_time TIMESTAMP,
    registrar             INTEGER,
    registrar_note        TEXT
);

CREATE TABLE person_name
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    user     INTEGER NOT NULL,
    title    TEXT    NOT NULL,
    forename TEXT    NOT NULL,
    surname  TEXT    NOT NULL,
    honours  TEXT
);

CREATE TABLE membership
(
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    user                 INTEGER,
    registered           TIMESTAMP,
    ceased               DATE,
    cessation_registered TIMESTAMP
);

CREATE TABLE user
(
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    username       TEXT NOT NULL UNIQUE,
    preferred_name TEXT NOT NULL,
    email          TEXT NOT NULL,
    irc            TEXT,
    github         TEXT
);
