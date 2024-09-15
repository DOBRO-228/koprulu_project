use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::description::{
    ActiveModel as DescriptionActiveModel, Entity as DescriptionEntity, Model as DescriptionModel,
    Column as DescriptionColumn,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, QueryOrder, TryIntoModel};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct DescriptionInput {
    pub description: String,
    pub meta_description: Option<String>,
    pub in_excess: Option<String>,
    pub in_norm: Option<String>,
    pub in_deficiency: Option<String>,
}

pub async fn get_all_descriptions(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<DescriptionModel>>), AppError> {
    let db = &*state.db;

    let descriptions = DescriptionEntity::find().order_by_asc(DescriptionColumn::Id)
        .all(db)
        .await
        .map_err(AppError::from)?;
    Ok((StatusCode::OK, Json(descriptions)))
}

pub async fn get_description(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let db = &*state.db;

    let description = DescriptionEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(AppError::from)?;
    if let Some(description) = description {
        Ok((StatusCode::OK, Json(json!(description))))
    } else {
        Ok((StatusCode::OK, Json(json!({}))))
    }
}

pub async fn create_description(
    State(state): State<AppState>,
    Json(input): Json<DescriptionInput>,
) -> Result<(StatusCode, Json<DescriptionModel>), AppError> {
    let db = &*state.db;

    let new_description = DescriptionActiveModel {
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

pub async fn update_description(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<DescriptionInput>,
) -> Result<(StatusCode, Json<DescriptionModel>), AppError> {
    let db = &*state.db;

    let new_description = DescriptionActiveModel {
        id: Set(id),
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

pub async fn delete_description(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let db = &*state.db;
    let description = DescriptionActiveModel {
        id: Set(id),
        ..Default::default()
    };
    description.delete(db).await.map_err(AppError::from)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_all_descriptions(
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    let db = &*state.db;

    DescriptionEntity::delete_many()
        .exec(db)
        .await
        .map_err(AppError::from)?;
    Ok(StatusCode::NO_CONTENT)
}
