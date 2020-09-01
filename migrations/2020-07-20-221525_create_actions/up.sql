-- Your SQL goes here

CREATE TABLE actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    device_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL UNIQUE,
    link VARCHAR(2100) NOT NULL UNIQUE,
    FOREIGN KEY(device_id) REFERENCES devices(id)
);

CREATE TABLE action_parameters (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    action_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    param_type VARCHAR(50) NOT NULL,
    min INTEGER,
    max INTEGER,
    FOREIGN KEY(action_id) REFERENCES actions(id)
);
