
extern crate rocket;
use rocket::*;
use super::endpoints::*;

pub fn mount_routes(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![index])
}
