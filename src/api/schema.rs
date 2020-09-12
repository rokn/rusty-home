table! {
    action_parameters (id) {
        id -> Integer,
        action_id -> Integer,
        name -> Text,
        param_type -> Text,
        min -> Nullable<Integer>,
        max -> Nullable<Integer>,
    }
}

table! {
    actions (id) {
        id -> Integer,
        device_id -> Integer,
        name -> Text,
        link -> Text,
    }
}

table! {
    devices (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    scene_actions (id) {
        id -> Integer,
        scene_id -> Integer,
        action_id -> Integer,
    }
}

table! {
    scene_parameters (scene_action_id, param_id) {
        scene_action_id -> Integer,
        param_id -> Integer,
        param_value -> Integer,
    }
}

table! {
    scenes (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(action_parameters -> actions (action_id));
joinable!(actions -> devices (device_id));
joinable!(scene_actions -> actions (action_id));
joinable!(scene_actions -> scenes (scene_id));
joinable!(scene_parameters -> action_parameters (param_id));
joinable!(scene_parameters -> scene_actions (scene_action_id));

allow_tables_to_appear_in_same_query!(
    action_parameters,
    actions,
    devices,
    scene_actions,
    scene_parameters,
    scenes,
);
