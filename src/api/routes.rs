
extern crate rocket;
use rocket::*;
use super::devices;
use super::scenes;
use super::automations;

pub fn mount_routes(rocket: Rocket) -> Rocket {
    rocket.mount("/api/v1", devices::routes::get_routes())
          .mount("/api/v1", scenes::routes::get_routes())
          .mount("/api/v1", automations::routes::get_routes())
}
