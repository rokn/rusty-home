use rocket::*;
use rocket::http::Status;

use crate::api::devices::controllers::*;
use crate::api::sqlite_db::SQLiteDbCtx;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/devices")]
pub fn devices_list(db: SQLiteDbCtx) -> Result<Template, Status> {
    let result = list_devices(&*db);
    return result.map(|devices| {
        let mut context = HashMap::new();
        context.insert("devices", devices);
        Template::render("devices", context)
    }).map_err(|_| Status::InternalServerError)
}

#[get("/devices/<id>")]
pub fn devices_get(id: i32, db: SQLiteDbCtx) -> Result<Template, Status> {
    let result = get_device_info(id, &*db);
    return result.map(|dev| Template::render("device", dev))
        .map_err(|_| Status::NotFound)
}

#[get("/devices/<device_id>/action/new")]
pub fn actions_create(device_id: i32) -> Result<Template, Status> {
    let mut context = HashMap::new();
    context.insert("device_id", device_id);
    return Ok(Template::render("action_new", context))
}
