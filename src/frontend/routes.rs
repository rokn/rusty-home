extern crate rocket;
use rocket::*;
use super::devices;
use super::scenes;

pub fn mount_routes(rocket: Rocket) -> Rocket {
    rocket.mount("/", devices::routes::get_routes())
        .mount("/", scenes::routes::get_routes())

}
