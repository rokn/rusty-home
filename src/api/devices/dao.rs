use crate::api::schema::devices;
use super::view_models::NewDevice;

#[derive(Queryable)]
pub struct DeviceDAO {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct ActionDAO {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct ParameterDAO {
    pub id: i32,
    pub name: String,
    pub param_type: String,
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Insertable)]
#[table_name="devices"]
pub struct NewDeviceDAO<'a> {
    pub name: &'a str
}

impl<'a> NewDeviceDAO<'a> {
    pub fn from_view_model(new_device: &'a NewDevice) -> Self {
        NewDeviceDAO { name: &new_device.name }
    }
}