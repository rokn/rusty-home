use rocket::*;
use rocket::http::Status;

use super::controllers::*;
use crate::api::sqlite_db::SQLiteDb;
use crate::api::devices::models::NewDevice;
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
pub fn devices_get(id: i32) -> Status {
    Status::NotImplemented
}

#[get("/devices/<device_id>/actions")]
pub fn actions_list(device_id: i32) -> Status {
    Status::NotImplemented
}

#[post("/devices/<device_id>/actions")]
pub fn actions_create(device_id: i32) -> Status {
    Status::NotImplemented
}

#[get("/devices/<device_id>/actions/<action_id>")]
pub fn actions_get(device_id: i32, action_id: i32) -> Status {
    Status::NotImplemented
}

#[post("/devices/<device_id>/actions/<action_id>/activate")]
pub fn actions_activate(device_id: i32, action_id: i32) -> Status {
    Status::NotImplemented
}
