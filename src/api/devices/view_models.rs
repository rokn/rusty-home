use serde::Deserialize;
use serde::Serialize;
use super::dao::*;

#[derive(Serialize)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub actions: Vec<Action>
}

impl Device {
    pub fn from_dao(device_dao: DeviceDAO, actions: Vec<Action>) -> Self {
        Device {
            id: device_dao.id,
            name: device_dao.name,
            actions
        }
    }
}

#[derive(Serialize)]
pub struct Action {
    pub id: i32,
    pub name: String,
    pub params: Vec<Parameter>
}

impl Action {
    pub fn from_dao(action_dao: ActionDAO, params: Vec<Parameter>) -> Self {
        Action {
            id: action_dao.id,
            name: action_dao.name,
            params
        }
    }
}

#[derive(Serialize)]
pub struct Parameter {
    pub id: i32,
    pub name: String,
    pub param_type: String,
    pub min: Option<i32>,
    pub max: Option<i32>,
}

impl Parameter {
    pub fn from_dao(param_dao: ParameterDAO) -> Self {
        Parameter {
            id: param_dao.id,
            name: param_dao.name,
            param_type: param_dao.param_type,
            min: param_dao.min,
            max: param_dao.max,
        }
    }
}


#[derive(Deserialize)]
pub struct NewDevice {
    pub name: String
}
