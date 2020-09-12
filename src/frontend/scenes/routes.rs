use rocket::Route;
use super::views::*;

pub fn get_routes() -> Vec<Route> {
    routes![
        scenes_list,
        scenes_get,
        scenes_create
    ]
}