use crate::generate_base_crud_handlers;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use common::app_state::AppState;
use common::errors::AppError;
use entity::food::{
    ActiveModel as FoodActiveModel, Column as FoodColumn, Entity as FoodEntity, Model as FoodModel,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FoodInput {
    pub name: String,
    pub description: String,
}

impl FoodInput {
    pub fn into_active_model(self) -> FoodActiveModel {
        FoodActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            ..Default::default()
        }
    }
}

generate_base_crud_handlers!(
    food,
    FoodEntity,
    FoodModel,
    FoodInput,
    FoodActiveModel,
    FoodColumn::Id
);
