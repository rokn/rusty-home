-- Your SQL goes here

CREATE TABLE actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE action_parameters (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(100) NOT NULL,
    param_type VARCHAR(50) NOT NULL,
    min INTEGER,
    max INTEGER
);

CREATE TABLE device_actions (
    device_id INTEGER,
    action_id INTEGER,
    FOREIGN KEY(device_id) REFERENCES devices(id),
    FOREIGN KEY(action_id) REFERENCES actions(id),
    PRIMARY KEY(device_id, action_id)
)
