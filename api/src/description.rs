use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::description::{self, Model as DescriptionModel};
use sea_orm::{EntityTrait, TryIntoModel};
use serde_json::json;
use service::Mutations;
use service::Queries;

pub async fn get_all_descriptions(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<DescriptionModel>>), AppError> {
    let db = &*state.db;

    let descriptions = Queries::get_all_entities::<description::Entity>(db)
        .await
        .map_err(AppError::from)?;
    Ok((StatusCode::OK, Json(descriptions)))
}

pub async fn get_description(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let db = &*state.db;

    let description = Queries::get_entity_by_id::<description::Entity>(db, id)
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
    Json(input): Json<DescriptionModel>,
) -> Result<(StatusCode, Json<description::Model>), AppError> {
    let db = &state.db;

    let form_data = DescriptionModel {
        id: 0,
        description: input.description,
        meta_description: input.meta_description,
        in_excess: input.in_excess,
        in_norm: input.in_norm,
        in_deficiency: input.in_deficiency,
    };

    let inserted_description = Mutations::create_description(db, form_data)
        .await
        .map_err(AppError::from)?;
    let description_model = inserted_description.try_into_model()?;
    Ok((StatusCode::OK, Json(description_model)))
}

pub async fn update_description(
    State(state): State<AppState>,
    Json(input): Json<DescriptionModel>,
) -> Result<(StatusCode, Json<description::Model>), AppError> {
    let db = &state.db;

    let form_data = DescriptionModel {
        id: 0,
        description: input.description,
        meta_description: input.meta_description,
        in_excess: input.in_excess,
        in_norm: input.in_norm,
        in_deficiency: input.in_deficiency,
    };

    let updated_active_model_description = Mutations::create_description(db, form_data)
        .await
        .map_err(AppError::from)?;
    let updated_model_description = updated_active_model_description.try_into_model()?;
    Ok((StatusCode::OK, Json(updated_model_description)))
}

pub async fn delete_description(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let db = &*state.db;

    let description = Queries::get_entity_by_id::<description::Entity>(db, id)
        .await
        .map_err(AppError::from)?;
    if let Some(model) = description {
        description::Entity::delete_by_id(model.id)
            .exec(db)
            .await
            .map_err(AppError::from)?;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_all_descriptions(
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    let db = &*state.db;

    Mutations::delete_all_entities::<description::Entity>(db)
        .await
        .map_err(AppError::from)?;
    Ok(StatusCode::NO_CONTENT)
}
