use rocket::*;
use rocket::http::Status;

#[get("/scenes")]
pub fn scenes_list() -> Status {
    Status::NotImplemented
}

#[post("/scenes")]
pub fn scenes_create() -> Status {
    Status::NotImplemented
}

#[get("/scenes/<id>")]
pub fn scenes_get(id: i32) -> Status {
    Status::NotImplemented
}

#[get("/scenes/<id>/activate")]
pub fn scenes_activate(id: i32) -> Status {
    Status::NotImplemented
}
