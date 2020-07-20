use rocket::*;
use rocket::http::Status;

#[get("/automations")]
pub fn automations_list() -> Status {
    Status::NotImplemented
}

#[post("/automations")]
pub fn automations_create() -> Status {
    Status::NotImplemented
}

#[get("/automations/<id>")]
pub fn automations_get(id: i32) -> Status {
    Status::NotImplemented
}

#[get("/automations/<id>/activate")]
pub fn automations_activate(id: i32) -> Status {
    Status::NotImplemented
}
