use rocket::Route;
use super::endpoints::*;

pub fn get_routes() -> Vec<Route> {
    routes![
        devices_list,
        devices_create,
        devices_get,

        actions_list,
        actions_create,
        actions_get,
        actions_activate,
    ]
}