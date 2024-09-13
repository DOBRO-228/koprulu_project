
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::hormone::{self, Model as hormoneModel};
use sea_orm::{EntityTrait, TryIntoModel};
use serde_json::json;
use service::Mutations;
use service::Queries;

pub async fn get_all_hormones(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<hormoneModel>>), AppError> {
    let db = &*state.db;

    let hormones = Queries::get_all_entities::<hormone::Entity>(db)
        .await
        .map_err(AppError::from)?;
    Ok((StatusCode::OK, Json(hormones)))
}

pub async fn get_hormone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let db = &*state.db;

    let hormone = Queries::get_entity_by_id::<hormone::Entity>(db, id)
        .await
        .map_err(AppError::from)?;
    if let Some(hormone) = hormone {
        Ok((StatusCode::OK, Json(json!(hormone))))
    } else {
        Ok((StatusCode::OK, Json(json!({}))))
    }
}

pub async fn create_hormone(
    State(state): State<AppState>,
    Json(input): Json<hormoneModel>,
) -> Result<(StatusCode, Json<hormone::Model>), AppError> {
    let db = &state.db;

    let form_data = hormoneModel {
        id: 0,
        name: input.name,
        hormone_type: input.hormone_type,
        description_id: input.description_id,
    };

    let inserted_hormone = Mutations::create_hormone(db, form_data)
        .await
        .map_err(AppError::from)?;
    let hormone_model = inserted_hormone.try_into_model()?;
    Ok((StatusCode::OK, Json(hormone_model)))
}

pub async fn update_hormone(
    State(state): State<AppState>,
    Json(input): Json<hormoneModel>,
) -> Result<(StatusCode, Json<hormone::Model>), AppError> {
    let db = &state.db;

    let form_data = hormoneModel {
        id: 0,
        hormone: input.hormone,
        meta_hormone: input.meta_hormone,
        in_excess: input.in_excess,
        in_norm: input.in_norm,
        in_deficiency: input.in_deficiency,
    };

    let updated_active_model_hormone = Mutations::create_hormone(db, form_data)
        .await
        .map_err(AppError::from)?;
    let updated_model_hormone = updated_active_model_hormone.try_into_model()?;
    Ok((StatusCode::OK, Json(updated_model_hormone)))
}

pub async fn delete_hormone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let db = &*state.db;

    let hormone = Queries::get_entity_by_id::<hormone::Entity>(db, id)
        .await
        .map_err(AppError::from)?;
    if let Some(model) = hormone {
        hormone::Entity::delete_by_id(model.id)
            .exec(db)
            .await
            .map_err(AppError::from)?;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_all_hormones(
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    let db = &*state.db;

    Mutations::delete_all_entities::<hormone::Entity>(db)
        .await
        .map_err(AppError::from)?;
    Ok(StatusCode::NO_CONTENT)
}
