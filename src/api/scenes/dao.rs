use crate::api::schema::*;
use super::view_models::*;

#[derive(Queryable)]
#[derive(Debug)]
pub struct SceneDao {
    pub id: i32,
    pub name: String,
}

// For grouping by
impl PartialEq for SceneDao {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Queryable)]
#[derive(Debug)]
pub struct SceneActionDao {
    pub id: i32,
    pub scene_id: i32,
    pub action_id: i32,
}

impl PartialEq for SceneActionDao {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Queryable)]
#[derive(Debug)]
pub struct SceneParameterDao {
    pub scene_action_id: i32,
    pub param_id: i32,
    pub param_value: i32,
}

#[derive(Insertable)]
#[derive(Debug)]
#[table_name="scenes"]
pub struct NewSceneDAO<'a> {
    pub name: &'a str
}

impl<'a> NewSceneDAO<'a> {
    pub fn from_view_model(new_scene: &'a NewScene) -> Self {
        NewSceneDAO { name: &new_scene.name }
    }
}

#[derive(Insertable)]
#[table_name="scene_parameters"]
pub struct NewSceneParameterDAO {
    pub scene_action_id: i32,
    pub param_id: i32,
    pub param_value: i32,
}

impl NewSceneParameterDAO {
    pub fn from_view_model(scene_action_id: i32, new_parameter: &NewSceneParameter) -> Self {
        NewSceneParameterDAO {
            scene_action_id,
            param_id: new_parameter.param_id,
            param_value: new_parameter.param_value,
        }
    }
}

#[derive(Insertable)]
#[table_name="scene_actions"]
pub struct NewSceneActionDAO {
    pub scene_id: i32,
    pub action_id: i32,
}

impl NewSceneActionDAO {
    pub fn from_view_model(scene_id: i32, new_action: &NewSceneAction) -> Self {
        NewSceneActionDAO {
            scene_id,
            action_id: new_action.action_id
        }
    }
}
