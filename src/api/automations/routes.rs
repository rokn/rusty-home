use rocket::Route;

use super::endpoints::*;

pub fn get_routes() -> Vec<Route> {
    routes![
        automations_list,
        automations_create,
        automations_get,
        automations_activate,
    ]
}