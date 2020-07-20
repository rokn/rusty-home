table! {
    action_parameters (id) {
        id -> Integer,
        name -> Text,
        param_type -> Text,
        min -> Nullable<Integer>,
        max -> Nullable<Integer>,
    }
}

table! {
    actions (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    device_actions (device_id, action_id) {
        device_id -> Nullable<Integer>,
        action_id -> Nullable<Integer>,
    }
}

table! {
    devices (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(device_actions -> actions (action_id));
joinable!(device_actions -> devices (device_id));

allow_tables_to_appear_in_same_query!(
    action_parameters,
    actions,
    device_actions,
    devices,
);
