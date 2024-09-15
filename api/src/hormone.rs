use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::hormone::{
    ActiveModel as HormoneActiveModel, Entity as HormoneEntity, Model as HormoneModel,
    Column as HormoneColumn, HormoneType,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, QueryOrder, TryIntoModel};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct HormoneInput {
    pub name: String,
    pub hormone_type: HormoneType,
    pub description: String,
    pub meta_description: Option<String>,
    pub in_excess: Option<String>,
    pub in_norm: Option<String>,
    pub in_deficiency: Option<String>,
}

pub async fn get_all_hormones(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<HormoneModel>>), AppError> {
    let db = &*state.db;

    let descriptions = HormoneEntity::find().order_by_asc(HormoneColumn::Id)
        .all(db)
        .await
        .map_err(AppError::from)?;
    Ok((StatusCode::OK, Json(descriptions)))
}

pub async fn get_hormone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let db = &*state.db;

    let description = HormoneEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(AppError::from)?;
    if let Some(description) = description {
        Ok((StatusCode::OK, Json(json!(description))))
    } else {
        Ok((StatusCode::OK, Json(json!({}))))
    }
}

pub async fn create_hormone(
    State(state): State<AppState>,
    Json(input): Json<HormoneInput>,
) -> Result<(StatusCode, Json<HormoneModel>), AppError> {
    let db = &*state.db;

    let new_description = HormoneActiveModel {
        description: Set(input.description),
        meta_description: Set(input.meta_description),
        in_excess: Set(input.in_excess),
        in_norm: Set(input.in_norm),
        in_deficiency: Set(input.in_deficiency),
        ..Default::default()
    };

    let created_description = new_description
        .save(db)
        .await
        .map_err(AppError::from)?
        .try_into_model()
        .map_err(AppError::from)?;
    Ok((StatusCode::CREATED, Json(created_description)))
}

pub async fn update_hormone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<HormoneInput>,
) -> Result<(StatusCode, Json<HormoneModel>), AppError> {
    let db = &*state.db;

    let new_description = HormoneActiveModel {
        id: Set(id),
        name: Set(input.name),
        hormone_type: Set(input.hormone_type),
        description: Set(input.description),
        meta_description: Set(input.meta_description),
        in_excess: Set(input.in_excess),
        in_norm: Set(input.in_norm),
        in_deficiency: Set(input.in_deficiency),
    };

    let updated_description = new_description.update(db).await.map_err(|e| match e {
        DbErr::RecordNotFound(msg) => AppError::ValidationError(msg.to_string()),
        other_err => AppError::from(other_err),
    })?;
    Ok((StatusCode::OK, Json(updated_description)))
}

pub async fn delete_hormone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let db = &*state.db;
    let description = HormoneActiveModel {
        id: Set(id),
        ..Default::default()
    };
    description.delete(db).await.map_err(AppError::from)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_all_hormones(
    State(_state): State<AppState>,
) -> Result<StatusCode, AppError> {
    // let db = &*state.db;
    // 
    // HormoneEntity::delete_many()
    //     .exec(db)
    //     .await
    //     .map_err(AppError::from)?;
    Ok(StatusCode::NO_CONTENT)
}
