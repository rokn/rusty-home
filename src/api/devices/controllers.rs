use crate::api::sqlite_db::SQLiteDb;
use super::dao::*;
use super::view_models::*;
use diesel::prelude::*;
use itertools::Itertools;

type FullDeviceSqlResult = Vec<(DeviceDAO, Option<(ActionDAO, Option<ParameterDAO>)>)>;
type FullDeviceSqlResultRef<'a> = Vec<&'a(DeviceDAO, Option<(ActionDAO, Option<ParameterDAO>)>)>;

type ActionSqlResultRef<'a> = Vec<&'a(ActionDAO, Option<ParameterDAO>)>;

pub fn create_device(new_device: NewDevice, db: SQLiteDb) -> QueryResult<usize> {
    use crate::api::schema::devices;

    diesel::insert_into(devices::table)
        .values(&NewDeviceDAO::from_view_model(&new_device))
        .execute(&*db)
}

pub fn get_device_info(device_id: i32, db: SQLiteDb) -> QueryResult<Device> {
    use crate::api::schema::devices::dsl::{devices as devices_table, id as id_field};
    use crate::api::schema::actions::dsl::{actions as actions_table};
    use crate::api::schema::action_parameters::dsl::{action_parameters as action_params_table};

    let joined_result: FullDeviceSqlResult =
        devices_table.left_join(actions_table.left_join(action_params_table))
            .filter(id_field.eq(device_id))
            .load(&*db)?;

    if joined_result.is_empty() {
        return Err(diesel::NotFound)
    }

    let actions = extract_actions(&joined_result.iter().collect());

    // We already made sure there is at least one
    let device_dao = &joined_result[0].0;
    Ok(Device::from_dao(device_dao, actions))
}

pub fn list_devices(db: SQLiteDb) -> QueryResult<Vec<Device>> {
    use crate::api::schema::devices::dsl::{devices as devices_table};
    use crate::api::schema::actions::dsl::{actions as actions_table};
    use crate::api::schema::action_parameters::dsl::{action_parameters as action_params_table};

    let joined_result: FullDeviceSqlResult =
        devices_table.left_join(actions_table.left_join(action_params_table))
            .load(&*db)?;

    if joined_result.is_empty() {
        return Err(diesel::NotFound)
    }

    let mut devices = Vec::new();
    for (key, group) in &joined_result.iter().group_by(|&(device, _)| device) {

        let actions = extract_actions(&group.into_iter().collect());
        // We already made sure there is at least one
        devices.push(Device::from_dao(&key, actions))
    }

    Ok(devices)
}

fn extract_actions(sql_result: &FullDeviceSqlResultRef) -> Vec<Action> {
    let action_daos: Vec<&(ActionDAO, Option<ParameterDAO>)> = sql_result.into_iter()
        .map(|(_, action)| action).flatten().collect();

    let mut actions = Vec::new();
    for (key, group) in &action_daos.into_iter().group_by(|&(action, _)| action) {
        let params = extract_params(&group.into_iter().collect());
        actions.push(Action::from_dao(key, params))
    }

    actions
}

fn extract_params(action_result: &ActionSqlResultRef) -> Vec<Parameter> {
    action_result.into_iter().map(|(_, param)| param)
        .flatten().map(Parameter::from_dao).collect()
}

pub fn create_action(device_id: i32, new_action: NewAction, db: SQLiteDb) -> QueryResult<usize> {
    use crate::api::schema::{actions, action_parameters};
    use crate::api::schema::actions::dsl::id;

    diesel::insert_into(actions::table)
        .values(&NewActionDAO::from_view_model(device_id, &new_action))
        .execute(&*db)?;
    let inserted_id: i32 = actions::table.select(id).order(id.desc()).first(&*db)?;

    let new_params: Vec<NewParameterDAO> = new_action.params
        .iter()
        .map(|new_parameter| NewParameterDAO::from_view_model(inserted_id, new_parameter))
        .collect();

    diesel::insert_into(action_parameters::table)
        .values(&new_params)
        .execute(&*db)
}

pub fn get_action(action_id: i32, db: SQLiteDb) -> QueryResult<Action> {
    use crate::api::schema::actions::dsl::{actions as actions_table, id as id_field};
    use crate::api::schema::action_parameters::dsl::{action_parameters as action_params_table};
    let sql_result: Vec<(ActionDAO, Option<ParameterDAO>)> = actions_table.left_join(action_params_table)
        .filter(id_field.eq(action_id)).load(&*db)?;
    if sql_result.is_empty() {
        return Err(diesel::NotFound)
    }

    let params = extract_params(&sql_result.iter().collect());
    let action_dao = &sql_result[0].0;
    Ok(Action::from_dao(action_dao, params))
}
