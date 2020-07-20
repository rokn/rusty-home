use rocket::Route;

use super::endpoints::*;

pub fn get_routes() -> Vec<Route> {
    routes![
        scenes_list,
        scenes_create,
        scenes_get,
        scenes_activate,
    ]
}