use super::hormone::{
    create_hormone, delete_hormone, get_all_hormones, get_hormone, update_hormone,
};
use axum::routing::get;
use axum::Router;
use common::app_state::AppState;
use crate::activity::{create_activity, delete_activity, get_activity, get_all_activitys, update_activity};
use crate::food::{create_food, delete_food, get_all_foods, get_food, update_food};
use crate::sport::{create_sport, delete_sport, get_all_sports, get_sport, update_sport};
use crate::supplement::{create_supplement, delete_supplement, get_all_supplements, get_supplement, update_supplement};
use crate::trigger::{create_trigger, delete_trigger, get_all_triggers, get_trigger, update_trigger};

pub fn get_api_routes() -> Router<AppState> {
    Router::new()
        .route("/hormones", get(get_all_hormones).post(create_hormone))
        .route(
            "/hormones/:id",
            get(get_hormone).put(update_hormone).delete(delete_hormone),
        )
        .route("/triggers", get(get_all_triggers).post(create_trigger))
        .route(
            "/triggers/:id",
            get(get_trigger).put(update_trigger).delete(delete_trigger),
        )
        .route("/activities", get(get_all_activitys).post(create_activity))
        .route(
            "/activities/:id",
            get(get_activity).put(update_activity).delete(delete_activity),
        )
        .route("/food", get(get_all_foods).post(create_food))
        .route(
            "/food/:id",
            get(get_food).put(update_food).delete(delete_food),
        )
        .route("/supplements", get(get_all_supplements).post(create_supplement))
        .route(
            "/supplements/:id",
            get(get_supplement).put(update_supplement).delete(delete_supplement),
        )
        .route("/sports", get(get_all_sports).post(create_sport))
        .route(
            "/sports/:id",
            get(get_sport).put(update_sport).delete(delete_sport),
        )
}
