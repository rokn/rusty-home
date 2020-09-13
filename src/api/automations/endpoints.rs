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

#[get("/automations/<_id>")]
pub fn automations_get(_id: i32) -> Status {
    Status::NotImplemented
}

#[get("/automations/<_id>/activate")]
pub fn automations_activate(_id: i32) -> Status {
    Status::NotImplemented
}
