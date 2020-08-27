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
    }
}

table! {
    devices (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(action_parameters -> actions (action_id));
joinable!(actions -> devices (device_id));

allow_tables_to_appear_in_same_query!(
    action_parameters,
    actions,
    devices,
);
