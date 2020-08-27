use crate::api::sqlite_db::SQLiteDb;
use super::dao::*;
use super::view_models::*;
use diesel::prelude::*;
use crate::api::schema::action_parameters::dsl::action_parameters;
use itertools::{Itertools, join};

pub fn create_device(new_device: NewDevice, db: SQLiteDb) -> QueryResult<usize> {
    use crate::api::schema::devices;

    diesel::insert_into(devices::table)
        .values(&NewDeviceDAO::from_view_model(&new_device))
        .execute(&*db)
}

pub fn get_device_info(device_id: i32, db: SQLiteDb) -> QueryResult<Device> {
    use crate::api::schema::devices::dsl::devices;
    use crate::api::schema::actions::dsl::{actions, device_id as device_id_field};

    let joined_result: Vec<(DeviceDAO, (ActionDAO, Option<ParameterDAO>))> =
        devices.inner_join(actions.left_join(action_parameters))
            .filter(device_id_field.eq(device_id))
            .load(&*db)?;

    if joined_result.is_empty() {
        return Err(diesel::NotFound)
    }

    let actions = joined_result.iter().map(|_, (acchmvtion))
    let device = Device::from_dao(&joined_result[0].0);
}

pub fn create_action(device_id: i32, new_action: NewAction, db: SQLiteDb) -> QueryResult<usize> {
    use crate::api::schema::{actions, action_parameters};
    use crate::api::schema::actions::dsl::id;

    diesel::insert_into(actions::table)
        .values(&NewActionDAO::from_view_model(device_id, &new_action))
        .execute(&*db);
    let inserted_id: i32 = actions::table.select(id).order(id.desc()).first(&*db)?;

    let new_params: Vec<NewParameterDAO> = new_action.params
        .iter()
        .map(|new_parameter| NewParameterDAO::from_view_model(inserted_id, new_parameter))
        .collect();

    diesel::insert_into(action_parameters::table)
        .values(&new_params)
        .execute(&*db)
}
