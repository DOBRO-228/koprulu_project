use crate::generate_base_crud_handlers;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::hormone::{
    ActiveModel as HormoneActiveModel, Column as HormoneColumn, Entity as HormoneEntity,
    HormoneType, Model as HormoneModel,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HormoneInput {
    pub name: String,
    pub description: String,
    pub meta_description: Option<String>,
    pub hormone_type: HormoneType,
    pub in_excess: Option<String>,
    pub in_norm: Option<String>,
    pub in_deficiency: Option<String>,
}

impl HormoneInput {
    pub fn into_active_model(self) -> HormoneActiveModel {
        HormoneActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            meta_description: Set(self.meta_description),
            hormone_type: Set(self.hormone_type),
            in_excess: Set(self.in_excess),
            in_norm: Set(self.in_norm),
            in_deficiency: Set(self.in_deficiency),
            ..Default::default()
        }
    }
}

generate_base_crud_handlers!(
    hormone,
    HormoneEntity,
    HormoneModel,
    HormoneInput,
    HormoneActiveModel,
    HormoneColumn::Id
);
