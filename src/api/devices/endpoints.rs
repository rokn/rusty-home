use rocket::*;
use rocket::http::Status;

use super::controllers::*;
use crate::api::sqlite_db::SQLiteDb;
use crate::api::devices::view_models::{NewDevice, Device, NewAction};
use rocket_contrib::json::Json;

#[get("/devices")]
pub fn devices_list() -> Status {
    Status::NotImplemented
}

#[post("/devices", format = "application/json", data = "<new_device>")]
pub fn devices_create(new_device: Json<NewDevice>, db: SQLiteDb) -> Status {
    let result = create_device(new_device.into_inner(), db);
    if result.is_err() {
        return Status::Conflict
    }

    return Status::Ok
}

#[get("/devices/<id>")]
pub fn devices_get(id: i32, db: SQLiteDb) -> Result<Json<Device>, Status> {
    let result = get_device_info(id, db);
    if result.is_err() {
        return Err(Status::NotFound)
    }

    return result.map(Json).map_err(|_| Status::InternalServerError)
}

#[get("/devices/<device_id>/actions")]
pub fn actions_list(device_id: i32, db: SQLiteDb) -> Status {
    Status::NotImplemented
}

#[post("/devices/<device_id>/actions", format = "application/json", data = "<new_action>")]
pub fn actions_create(device_id: i32, new_action: Json<NewAction>, db: SQLiteDb) -> Status {
    let result = create_action( device_id, new_action.into_inner(), db);
    if result.is_err() {
        return Status::Conflict
    }

    return Status::Ok
}

#[get("/devices/<device_id>/actions/<action_id>")]
pub fn actions_get(device_id: i32, action_id: i32) -> Status {
    Status::NotImplemented
}

#[post("/devices/<device_id>/actions/<action_id>/activate")]
pub fn actions_activate(device_id: i32, action_id: i32) -> Status {
    Status::NotImplemented
}
