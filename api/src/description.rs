use axum::extract::{Extension, Json};
use entity::{description, description::Entity as Description};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use service::mutation::Mutation;


#[derive(Deserialize, Serialize)]
struct CreateDescription {
    description: String,
    meta_description: Option<String>,
    in_excess: Option<String>,
    in_norm: Option<String>,
    in_deficiency: Option<String>,
}

pub async fn create_description_handler(
    Json(payload): Json<CreateDescription>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let form_data = description::Model {
        id: 0,
        description: payload.description,
        meta_description: payload.meta_description,
        in_excess: payload.in_excess,
        in_norm: payload.in_norm,
        in_deficiency: payload.in_deficiency,
    };

    match Mutation::create_description(&db, form_data).await {
        Ok(saved_description) => (
            StatusCode::CREATED,
            Json(saved_description)
        ).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create description: {}", err),
        ).into_response(),
    }
}