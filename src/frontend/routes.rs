extern crate rocket;
use rocket::*;
use super::devices;

pub fn mount_routes(rocket: Rocket) -> Rocket {
    rocket.mount("/", devices::routes::get_routes())
}
