-- Your SQL goes here
CREATE TABLE scenes (
     id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
     name VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE scene_actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    scene_id INTEGER NOT NULL,
    action_id INTEGER NOT NULL,
    FOREIGN KEY (scene_id) REFERENCES scenes(id),
    FOREIGN KEY (action_id) REFERENCES actions(id)
);

CREATE TABLE scene_parameters (
   scene_action_id INTEGER NOT NULL,
   param_id INTEGER NOT NULL,
   param_value INTEGER NOT NULL,
   PRIMARY KEY (scene_action_id, param_id),
   FOREIGN KEY (scene_action_id) REFERENCES scene_actions(id),
   FOREIGN KEY (param_id) REFERENCES action_parameters(id)
);
