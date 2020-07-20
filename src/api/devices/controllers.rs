use crate::api::sqlite_db::SQLiteDb;
use super::models::*;
use crate::api::schema::devices::dsl::devices as dev;
use crate::api::schema::devices;
use diesel::prelude::*;

pub fn create_device(new_device: NewDevice, db: SQLiteDb) -> QueryResult<usize> {
    diesel::insert_into(devices::table)
        .values(&new_device)
        .execute(&*db)
}