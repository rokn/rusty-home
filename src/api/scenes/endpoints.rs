use rocket::*;
use rocket::http::Status;
use crate::api::scenes::view_models::{NewScene, Scene, NewSceneAction};
use rocket_contrib::json::Json;
use crate::api::sqlite_db::SQLiteDbCtx;
use crate::api::scenes::controllers::{create_scene, list_scenes, get_scene_info, activate_scene, append_action};

#[get("/scenes")]
pub fn scenes_list(db: SQLiteDbCtx) -> Result<Json<Vec<Scene>>, Status> {
    let result = list_scenes(&*db);
    return result.map(Json).map_err(|_| Status::InternalServerError)
}

#[post("/scenes", format = "application/json", data = "<new_scene>")]
pub fn scenes_create(new_scene: Json<NewScene>, db: SQLiteDbCtx) -> Result<(), Status> {
    let result = create_scene(new_scene.into_inner(), &*db);
    return result.map(|_| ()).map_err(|_| Status::Conflict)
}

#[post("/scenes/<scene_id>/append", format = "application/json", data = "<new_action>")]
pub fn scenes_append_action(scene_id: i32, new_action: Json<NewSceneAction>, db: SQLiteDbCtx) -> Result<(), Status> {
    let result = append_action(scene_id, new_action.into_inner(), &*db);
    return result.map(|_| ()).map_err(|_| Status::InternalServerError)
}

#[get("/scenes/<id>")]
pub fn scenes_get(id: i32, db: SQLiteDbCtx) -> Result<Json<Scene>, Status> {
    let result = get_scene_info(id, &*db);
    return result.map(Json).map_err(|_| Status::NotFound)
}

#[post("/scenes/<id>/activate")]
pub fn scenes_activate(id: i32, db: SQLiteDbCtx) -> Result<Status, Status> {
    activate_scene(id, &*db)
        .map_err(|_| Status::InternalServerError)
}
