use crate::generate_base_crud_handlers;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::activity::{
    ActiveModel as ActivityActiveModel, Column as ActivityColumn, Entity as ActivityEntity,
    Model as ActivityModel,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ActivityInput {
    pub name: String,
    pub description: String,
}

impl ActivityInput {
    pub fn into_active_model(self) -> ActivityActiveModel {
        ActivityActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            ..Default::default()
        }
    }
}

generate_base_crud_handlers!(
    activity,
    ActivityEntity,
    ActivityModel,
    ActivityInput,
    ActivityActiveModel,
    ActivityColumn::Id
);
