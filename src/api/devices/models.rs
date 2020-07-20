use crate::api::schema::devices;
use serde::Deserialize;

#[derive(Queryable)]
pub struct Device {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[table_name="devices"]
pub struct NewDevice<'a> {
    pub name: &'a str
}