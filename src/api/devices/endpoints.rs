use std::collections::HashMap;

use rocket::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::api::devices::view_models::{Action, Device, NewAction, NewDevice};
use crate::api::sqlite_db::SQLiteDbCtx;

use super::controllers::*;

#[get("/devices")]
pub fn devices_list(db: SQLiteDbCtx) -> Result<Json<Vec<Device>>, Status> {
    let result = list_devices(&*db);
    return result.map(Json).map_err(|_| Status::InternalServerError)
}

#[post("/devices", format = "application/json", data = "<new_device>")]
pub fn devices_create(new_device: Json<NewDevice>, db: SQLiteDbCtx) -> Result<(), Status> {
    let result = create_device(new_device.into_inner(), &*db);
    return result.map(|_| ()).map_err(|_| Status::Conflict)
}

#[get("/devices/<id>")]
pub fn devices_get(id: i32, db: SQLiteDbCtx) -> Result<Json<Device>, Status> {
    let result = get_device_info(id, &*db);
    return result.map(Json).map_err(|_| Status::NotFound)
}

#[post("/devices/<device_id>/actions", format = "application/json", data = "<new_action>")]
pub fn actions_create(device_id: i32, new_action: Json<NewAction>, db: SQLiteDbCtx) -> Status {
    let result = create_action( device_id, new_action.into_inner(), &*db);
    return result.map(|_| Status::Ok).unwrap_or(Status::Conflict);
}

#[get("/actions/<action_id>")]
pub fn actions_get(action_id: i32, db: SQLiteDbCtx) -> Result<Json<Action>, Status> {
    let result = get_action(action_id, &*db);
    return result.map(Json).map_err(|_| Status::NotFound)
}

#[delete("/actions/<action_id>")]
pub fn actions_delete(action_id: i32, db: SQLiteDbCtx) -> Result<(), Status> {
    let result = delete_action(action_id, &*db);
    return result.map(|_| ()).map_err(|_| Status::NotFound)
}

#[post("/actions/<action_id>/activate", format = "application/json", data = "<params>")]
pub fn actions_activate(action_id: i32, params: Json<HashMap<String, i32>>, db: SQLiteDbCtx) -> Result<Status, Status> {
    activate_action(action_id, params.0, &*db)
        .map_err(|_| Status::InternalServerError)
}
