use rocket::*;
use rocket::http::Status;

use super::controllers::*;
use crate::api::sqlite_db::SQLiteDb;
use crate::api::devices::view_models::{NewDevice, Device, NewAction, Action};
use rocket_contrib::json::Json;

#[get("/devices")]
pub fn devices_list(db: SQLiteDb) -> Result<Json<Vec<Device>>, Status> {
    let result = list_devices(db);
    return result.map(Json).map_err(|_| Status::InternalServerError)
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
    return result.map(Json).map_err(|_| Status::NotFound)
}

#[post("/devices/<device_id>/actions", format = "application/json", data = "<new_action>")]
pub fn actions_create(device_id: i32, new_action: Json<NewAction>, db: SQLiteDb) -> Status {
    let result = create_action( device_id, new_action.into_inner(), db);
    return result.map(|_| Status::Ok).unwrap_or(Status::Conflict);
}

#[get("/actions/<action_id>")]
pub fn actions_get(action_id: i32, db: SQLiteDb) -> Result<Json<Action>, Status> {
    let result = get_action(action_id, db);
    return result.map(Json).map_err(|_| Status::NotFound)
}

#[post("/actions/<action_id>/activate")]
pub fn actions_activate(action_id: i32) -> Status {
    Status::NotImplemented
}
