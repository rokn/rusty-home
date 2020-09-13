use rocket::Route;
use super::views::*;

pub fn get_routes() -> Vec<Route> {
    routes![
        devices_list,
        devices_get,
        actions_create,
        devices_create,
    ]
}