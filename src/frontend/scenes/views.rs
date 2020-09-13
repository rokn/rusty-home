use rocket::*;
use rocket::http::Status;

use crate::api::scenes::controllers::*;
use crate::api::sqlite_db::SQLiteDbCtx;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::templates::tera::Context;

#[get("/scenes")]
pub fn scenes_list(db: SQLiteDbCtx) -> Result<Template, Status> {
    let result = list_scenes(&*db);
    return result.map(|scenes| {
        let mut context = HashMap::new();
        context.insert("scenes", scenes);
        Template::render("scenes", context)
    }).map_err(|_| Status::InternalServerError)
}

#[get("/scenes/<id>")]
pub fn scenes_get(id: i32, db: SQLiteDbCtx) -> Result<Template, Status> {
    let result = get_scene_info(id, &*db);
    return result.map(|scene| Template::render("scene", scene))
        .map_err(|_| Status::NotFound)
}

#[get("/scenes/new")]
pub fn scenes_create() -> Result<Template, Status> {
    return Ok(Template::render("scene_new", Context::new()))
}

#[get("/scenes/<id>/append")]
pub fn scenes_append(id: i32, db: SQLiteDbCtx) -> Result<Template, Status> {
    let vm = get_scene_action_add_vm(id, &*db);
    vm.map(|vm| Template::render("scene_append", vm))
        .map_err(|_| Status::InternalServerError)
}
