use crate::generate_base_crud_handlers;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::trigger::{
    ActiveModel as TriggerActiveModel, Column as TriggerColumn, Entity as TriggerEntity,
    Model as TriggerModel,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TriggerInput {
    pub name: String,
    pub description: String,
}

impl TriggerInput {
    pub fn into_active_model(self) -> TriggerActiveModel {
        TriggerActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            ..Default::default()
        }
    }
}

generate_base_crud_handlers!(
    trigger,
    TriggerEntity,
    TriggerModel,
    TriggerInput,
    TriggerActiveModel,
    TriggerColumn::Id
);
