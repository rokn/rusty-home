use crate::api::sqlite_db::SQLiteDb;
use super::dao::*;
use super::view_models::*;
use crate::api::schema::devices::dsl::devices as devices_dsl;
use crate::api::schema::{devices, actions};
use diesel::prelude::*;

pub fn create_device(new_device: NewDevice, db: SQLiteDb) -> QueryResult<usize> {
    diesel::insert_into(devices::table)
        .values(&NewDeviceDAO::from_view_model(&new_device))
        .execute(&*db)
}

pub fn get_device_info(device_id: i32, db: SQLiteDb) -> QueryResult<Device> {
    let device: DeviceDAO = devices_dsl.find(device_id).first(&*db)?;
    return Ok(Device::from_dao(device, vec![]))
}