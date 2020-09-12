use crate::api::sqlite_db::SQLiteDb;
use super::dao::*;
use super::view_models::*;
use rocket::http::Status;
use diesel::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;
use reqwest;
use crate::api::errors::ControllerError;

type FullDeviceSqlResult = Vec<(DeviceDAO, Option<(ActionDAO, Option<ParameterDAO>)>)>;
type FullDeviceSqlResultRef<'a> = Vec<&'a(DeviceDAO, Option<(ActionDAO, Option<ParameterDAO>)>)>;

type ActionSqlResultRef<'a> = Vec<&'a(ActionDAO, Option<ParameterDAO>)>;

pub type ControllerResult<T> = Result<T, ControllerError>;

pub fn create_device(new_device: NewDevice, db: SQLiteDb) -> ControllerResult<()> {
    use crate::api::schema::devices;

    diesel::insert_into(devices::table)
        .values(&NewDeviceDAO::from_view_model(&new_device))
        .execute(db)?;
    return Ok(())
}

pub fn get_device_info(device_id: i32, db: SQLiteDb) -> ControllerResult<Device> {
    use crate::api::schema::devices::dsl::{devices as devices_table, id as id_field};
    use crate::api::schema::actions::dsl::{actions as actions_table};
    use crate::api::schema::action_parameters::dsl::{action_parameters as action_params_table};

    let joined_result: FullDeviceSqlResult =
        devices_table.left_join(actions_table.left_join(action_params_table))
            .filter(id_field.eq(device_id))
            .load(db)?;

    if joined_result.is_empty() {
        return Err(ControllerError::NotFound)
    }

    let actions = extract_actions(&joined_result.iter().collect());

    // We already made sure there is at least one
    let device_dao = &joined_result[0].0;
    Ok(Device::from_dao(device_dao, actions))
}

pub fn list_devices(db: SQLiteDb) -> ControllerResult<Vec<Device>> {
    use crate::api::schema::devices::dsl::{devices as devices_table};
    use crate::api::schema::actions::dsl::{actions as actions_table};
    use crate::api::schema::action_parameters::dsl::{action_parameters as action_params_table};

    let joined_result: FullDeviceSqlResult =
        devices_table.left_join(actions_table.left_join(action_params_table))
            .load(db)?;

    if joined_result.is_empty() {
        return Err(ControllerError::NotFound)
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

pub fn create_action(device_id: i32, new_action: NewAction, db: SQLiteDb) -> ControllerResult<()> {
    use crate::api::schema::{actions, action_parameters};
    use crate::api::schema::actions::dsl::id;

    let allowed_types = vec!["range", "percentage"];
    let has_bad_types = new_action.params.iter()
        .any(|param| !allowed_types.contains(&&*param.param_type));
    if has_bad_types {
        return Err(ControllerError::ValidationError);
    }

    diesel::insert_into(actions::table)
        .values(&NewActionDAO::from_view_model(device_id, &new_action))
        .execute(db)?;
    let inserted_id: i32 = actions::table.select(id).order(id.desc()).first(db)?;

    let new_params: Vec<NewParameterDAO> = new_action.params
        .iter()
        .map(|new_parameter| NewParameterDAO::from_view_model(inserted_id, new_parameter))
        .collect();

    diesel::insert_into(action_parameters::table)
        .values(&new_params)
        .execute(db)?;
    return Ok(());
}

pub fn get_action(action_id: i32, db: SQLiteDb) -> ControllerResult<Action> {
    use crate::api::schema::actions::dsl::{actions as actions_table, id as id_field};
    use crate::api::schema::action_parameters::dsl::{action_parameters as action_params_table};
    let sql_result: Vec<(ActionDAO, Option<ParameterDAO>)> = actions_table.left_join(action_params_table)
        .filter(id_field.eq(action_id)).load(db)?;
    if sql_result.is_empty() {
        return Err(ControllerError::NotFound)
    }

    let params = extract_params(&sql_result.iter().collect());
    let action_dao = &sql_result[0].0;
    Ok(Action::from_dao(action_dao, params))
}

pub fn delete_action(action_id: i32, db: SQLiteDb) -> ControllerResult<()> {
    use crate::api::schema::actions::dsl::{actions as actions_table};
    use crate::api::schema::action_parameters::dsl::{action_parameters as action_params_table, action_id as action_id_field};

    // TODO: make into a transaction
    diesel::delete(actions_table.find(action_id)).execute(db)?;
    diesel::delete(action_params_table.filter(action_id_field.eq(action_id))).execute(db)?;

    Ok(())
}

pub fn activate_action(action_id: i32, params: HashMap<String, i32>, db: SQLiteDb) -> ControllerResult<Status> {
    let action = get_action(action_id, db)?;

    let mut action_url = action.link;
    for param in action.params {
        let val = *params.get(param.name.as_str()).ok_or(ControllerError::WrongActionParameters)?;
        action_url = action_url.replace(format!("<{}>", param.name).as_str(), val.to_string().as_str());
    }

    let response = reqwest::blocking::get(action_url.as_str()).map_err(|_| ControllerError::CantCallActionLink)?;

    // Hope for the best with this unwrap
    Ok(Status::from_code(response.status().as_u16()).unwrap())
}
