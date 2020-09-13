use crate::api::sqlite_db::SQLiteDb;
use super::dao::*;
use super::view_models::*;
use diesel::prelude::*;
use rocket::http::Status;
use itertools::Itertools;
use crate::api::devices::controllers::{get_action, activate_action, list_devices};
use crate::api::errors::ControllerError;
use std::collections::HashMap;

type FullSceneSqlResult = Vec<(SceneDao, Option<(SceneActionDao, Option<SceneParameterDao>)>)>;
type FullSceneSqlResultRef<'a> = Vec<&'a(SceneDao, Option<(SceneActionDao, Option<SceneParameterDao>)>)>;

type SceneActionSqlResultRef<'a> = Vec<&'a(SceneActionDao, Option<SceneParameterDao>)>;

pub type ControllerResult<T> = Result<T, ControllerError>;

pub fn create_scene(new_scene: NewScene, db: SQLiteDb) -> ControllerResult<()> {
    use crate::api::schema::scenes;
    // use crate::api::schema::scenes::dsl::{id as scene_id_field};

    let new_scene_dao = NewSceneDAO::from_view_model(&new_scene);
    diesel::insert_into(scenes::table)
        .values(&new_scene_dao)
        .execute(&*db)?;
    return Ok(())
}

pub fn append_action(scene_id: i32, new_action: NewSceneAction, db: SQLiteDb) -> ControllerResult<()> {
    use crate::api::schema::scene_actions;
    use crate::api::schema::scene_actions::dsl::{id as scene_action_id_field};
    use crate::api::schema::scene_parameters;

    let new_action_dao = NewSceneActionDAO::from_view_model(scene_id, &new_action);
    diesel::insert_into(scene_actions::table)
        .values(&new_action_dao)
        .execute(&*db)?;
    let scene_action_id: i32 = scene_actions::table.select(scene_action_id_field).order(scene_action_id_field.desc()).first(&*db)?;

    //TODO: Check if params are within constraints
    let param_values: Vec<NewSceneParameterDAO> = new_action.param_values.iter()
        .map(|new_param| NewSceneParameterDAO::from_view_model(scene_action_id,new_param))
        .collect();

    diesel::insert_into(scene_parameters::table)
        .values(&param_values)
        .execute(&*db)?;

    return Ok(())
}

pub fn list_scenes(db: SQLiteDb) -> ControllerResult<Vec<Scene>> {
    use crate::api::schema::scenes::dsl::{scenes as scenes_table};
    use crate::api::schema::scene_actions::dsl::{scene_actions as scene_actions_table};
    use crate::api::schema::scene_parameters::dsl::{scene_parameters as scene_params_table};

    let joined_result: FullSceneSqlResult =
        scenes_table.left_join(scene_actions_table.left_join(scene_params_table))
            .load(&*db)?;

    if joined_result.is_empty() {
        return Err(ControllerError::NotFound)
    }

    let mut scenes = Vec::new();
    for (key, group) in &joined_result.iter().group_by(|&(scene, _)| scene) {
        let actions = extract_actions(&group.into_iter().collect(), &db)?;
        // We already made sure there is at least one
        scenes.push(Scene::from_dao(key, actions))
    }

    return Ok(scenes)
}

fn extract_actions(sql_result: &FullSceneSqlResultRef, db: SQLiteDb) -> ControllerResult<Vec<SceneAction>> {
    let action_daos: SceneActionSqlResultRef = sql_result.into_iter()
        .map(|(_, action)| action).flatten().collect();

    let mut actions = Vec::new();
    for (key, group) in &action_daos.into_iter().group_by(|&(action, _)| action) {
        let params = extract_params(&group.into_iter().collect());
        let action = get_action(key.action_id, db)?;
        actions.push(SceneAction::from_dao(action, params));
    }

    Ok(actions)
}

fn extract_params(action_result: &SceneActionSqlResultRef) -> Vec<SceneParameter> {
    action_result.iter().map(|(_, param)| param).flatten()
        .map(SceneParameter::from_dao).collect()
}

pub fn get_scene_info(scene_id: i32, db: SQLiteDb) -> ControllerResult<Scene> {
    use crate::api::schema::scenes::dsl::{scenes as scenes_table, id as id_field};
    use crate::api::schema::scene_actions::dsl::{scene_actions as scene_actions_table};
    use crate::api::schema::scene_parameters::dsl::{scene_parameters as scene_params_table};

    let joined_result: FullSceneSqlResult =
        scenes_table.left_join(scene_actions_table.left_join(scene_params_table))
            .filter(id_field.eq(scene_id))
            .load(db)?;

    if joined_result.is_empty() {
        return Err(ControllerError::NotFound)
    }

    let actions = extract_actions(&joined_result.iter().collect(), db)?;

    // We already made sure there is at least one
    let scene_dao = &joined_result[0].0;
    Ok(Scene::from_dao(scene_dao, actions))
}

pub fn activate_scene(scene_id: i32, db: SQLiteDb) -> ControllerResult<Status> {
    let scene = get_scene_info(scene_id, db)?;

    for action in scene.actions {
        let param_names: HashMap<_, _> = action.real_action.params.into_iter()
            .map(|p| (p.id, p.name)).collect();
        let param_values = action.parameter_values.iter()
            .map(|p| (param_names[&p.param_id].clone(), p.param_value)).collect();
        let res = activate_action(action.real_action.id, param_values, db)?;
        if res != Status::Ok {
            return Ok(res)
        }
    }

    Ok(Status::Ok)
}

pub fn get_scene_action_add_vm(scene_id: i32, db: SQLiteDb) -> ControllerResult<SceneActionAdd> {
    let devices = list_devices(db)?;
    Ok(SceneActionAdd {
        scene_id,
        devices,
    })
}

pub fn delete_scene(scene_id: i32, db: SQLiteDb) -> ControllerResult<()> {
    use crate::api::schema::scenes::dsl::{scenes as scenes_table, id};
    use crate::api::schema::scene_actions::dsl::{scene_actions as scene_actions_table, scene_id as scene_id_field, id as id_field};
    use crate::api::schema::scene_parameters::dsl::{scene_parameters as scene_parameters_table, scene_action_id};

    // TODO: make into a transaction
    let scene_action_ids: Vec<i32> = scene_actions_table
        .select(id_field)
        .filter(scene_id_field.eq(scene_id))
        .load(db)?;

    diesel::delete(scene_parameters_table.filter(scene_action_id.eq_any(scene_action_ids))).execute(db)?;
    diesel::delete(scene_actions_table.filter(scene_id_field.eq(scene_id))).execute(db)?;
    diesel::delete(scenes_table.filter(id.eq(scene_id))).execute(db)?;

    Ok(())
}
