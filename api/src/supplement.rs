use crate::generate_base_crud_handlers;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::supplement::{
    ActiveModel as SupplementActiveModel, Column as SupplementColumn, Entity as SupplementEntity,
    Model as SupplementModel,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SupplementInput {
    pub name: String,
    pub description: String,
    pub dosage_and_administration: Option<String>,
}

impl SupplementInput {
    pub fn into_active_model(self) -> SupplementActiveModel {
        SupplementActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            dosage_and_administration: Set(self.dosage_and_administration),
            ..Default::default()
        }
    }
}

generate_base_crud_handlers!(
    supplement,
    SupplementEntity,
    SupplementModel,
    SupplementInput,
    SupplementActiveModel,
    SupplementColumn::Id
);
