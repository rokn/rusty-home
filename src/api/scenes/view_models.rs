use serde::Deserialize;
use serde::Serialize;
use super::dao::*;
use crate::api::devices::view_models::Action;

#[derive(Serialize)]
pub struct Scene {
    pub id: i32,
    pub name: String,
    pub actions: Vec<SceneAction>
}

impl Scene {
    pub fn from_dao(scene_dao: &SceneDao, actions: Vec<SceneAction>) -> Self {
        Scene {
            id: scene_dao.id,
            name: scene_dao.name.clone(),
            actions
        }
    }
}

#[derive(Serialize)]
pub struct SceneAction {
    pub real_action: Action,
    pub parameter_values: Vec<SceneParameter>
}

impl SceneAction {
    pub fn from_dao(real_action: Action, parameter_values: Vec<SceneParameter>) -> Self {
        SceneAction {
            real_action,
            parameter_values
        }
    }
}

#[derive(Serialize)]
pub struct SceneParameter {
    pub param_id: i32,
    pub param_value: i32
}

impl SceneParameter {
    pub fn from_dao(scene_param_dao: &SceneParameterDao) -> Self {
        SceneParameter {
            param_id: scene_param_dao.param_id,
            param_value: scene_param_dao.param_value
        }
    }
}

#[derive(Deserialize)]
pub struct NewScene {
    pub name: String,
    pub actions: Vec<NewSceneAction>
}

#[derive(Deserialize)]
pub struct NewSceneParameter {
    pub param_id: i32,
    pub param_value: i32,
}

#[derive(Deserialize)]
pub struct NewSceneAction {
    pub action_id: i32,
    pub param_values: Vec<NewSceneParameter>

}

#[derive(Deserialize)]
pub struct AppendSceneAction {
    pub scene_id: i32,
    pub action_id: i32,
    pub param_values: Vec<NewSceneParameter>

}
