-- Add migration script here
CREATE TABLE users (
    id          INTEGER PRIMARY KEY,
    created_at  DATETIME NOT NULL,
    admin       BOOLEAN NOT NULL,
    name        TEXT NOT NULL,
    password    BLOB NOT NULL
);