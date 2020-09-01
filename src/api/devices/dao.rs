use crate::api::schema::*;
use super::view_models::*;

#[derive(Queryable)]
#[derive(Debug)]
pub struct DeviceDAO {
    pub id: i32,
    pub name: String,
}

impl PartialEq for DeviceDAO {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Queryable)]
#[derive(Debug)]
pub struct ActionDAO {
    pub id: i32,
    pub device_id: i32,
    pub name: String,
    pub link: String,
}

impl PartialEq for ActionDAO {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.device_id == other.device_id
    }
}

#[derive(Queryable)]
#[derive(Debug)]
pub struct ParameterDAO {
    pub id: i32,
    pub action_id: i32,
    pub name: String,
    pub param_type: String,
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Insertable)]
#[derive(Debug)]
#[table_name="devices"]
pub struct NewDeviceDAO<'a> {
    pub name: &'a str
}

impl<'a> NewDeviceDAO<'a> {
    pub fn from_view_model(new_device: &'a NewDevice) -> Self {
        NewDeviceDAO { name: &new_device.name }
    }
}


#[derive(Insertable)]
#[table_name="action_parameters"]
pub struct NewParameterDAO<'a> {
    pub action_id: i32,
    pub name: &'a str,
    pub param_type: &'a str,
    pub min: Option<i32>,
    pub max: Option<i32>,
}

impl<'a> NewParameterDAO<'a> {
    pub fn from_view_model(action_id: i32, new_parameter: &'a NewParameter) -> Self {
        NewParameterDAO {
            action_id,
            name: &new_parameter.name,
            param_type: &new_parameter.param_type,
            min: new_parameter.min,
            max: new_parameter.max,
        }
    }
}

#[derive(Insertable)]
#[table_name="actions"]
pub struct NewActionDAO<'a> {
    pub device_id: i32,
    pub name: &'a str,
    pub link: &'a str,
}

impl<'a> NewActionDAO<'a> {
    pub fn from_view_model(device_id: i32, new_action: &'a NewAction) -> Self {
        NewActionDAO {
            device_id,
            name: &new_action.name,
            link: &new_action.link
        }
    }
}
