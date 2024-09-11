use axum::{Json, extract::State};
use axum::extract::Path;
use axum::http::StatusCode;
use sea_orm::{EntityTrait, TryIntoModel};
use service::Mutations;
use service::Queries;
use entity::description::{self, Model as DescriptionModel};
use common::app_state::AppState;
use common::errors::AppError;

#[derive(serde::Deserialize)]
pub struct CreateDescription {
    pub description: String,
    pub meta_description: Option<String>,
    pub in_excess: Option<String>,
    pub in_norm: Option<String>,
    pub in_deficiency: Option<String>,
}

pub async fn create_description(
    State(state): State<AppState>,
    Json(input): Json<CreateDescription>,
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

    let inserted_description = Mutations::create_description(db, form_data).await?;

    let description_model = inserted_description.try_into_model()?; 
    Ok((StatusCode::OK, Json(description_model)))
}

pub async fn delete_description(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let db = &*state.db;

    let description = Queries::find_entity_by_id::<description::Entity>(db, id).await;
    if let Some(model) = description {
        description::Entity::delete_by_id(model.id).exec(db).await.map_err(AppError::from)?;
    }

    Ok(StatusCode::NO_CONTENT)
}