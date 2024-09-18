use crate::generate_base_crud_handlers;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::supplement::{
    ActiveModel as SportActiveModel, Column as SportColumn, Entity as SportEntity,
    Model as SportModel,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SportInput {
    pub name: String,
    pub description: String,
}

impl SportInput {
    pub fn into_active_model(self) -> SportActiveModel {
        SportActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            ..Default::default()
        }
    }
}

generate_base_crud_handlers!(
    sport,
    SportEntity,
    SportModel,
    SportInput,
    SportActiveModel,
    SportColumn::Id
);
