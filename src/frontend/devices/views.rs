use rocket::*;
use rocket::http::Status;

use crate::api::devices::controllers::*;
use crate::api::sqlite_db::SQLiteDb;
use crate::api::devices::view_models::{NewDevice, Device, NewAction, Action};
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/devices")]
pub fn devices_list(db: SQLiteDb) -> Result<Template, Status> {
    let result = list_devices(db);
    return result.map(|devices| {
        let mut context = HashMap::new();
        context.insert("devices", devices);
        Template::render("devices", context)
    }).map_err(|_| Status::InternalServerError)
}

#[get("/devices/<id>")]
pub fn devices_get(id: i32, db: SQLiteDb) -> Result<Template, Status> {
    let result = get_device_info(id, db);
    return result.map(|dev| Template::render("device", dev))
        .map_err(|_| Status::NotFound)
}

#[get("/actions/<action_id>")]
pub fn actions_get(action_id: i32, db: SQLiteDb) -> Result<Json<Action>, Status> {
    let result = get_action(action_id, db);
    return result.map(Json).map_err(|_| Status::NotFound)
}
