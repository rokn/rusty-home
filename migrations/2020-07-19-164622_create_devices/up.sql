-- Your SQL goes here

CREATE TABLE devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(100) NOT NULL UNIQUE
);