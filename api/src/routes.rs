use super::description::{
    create_description, delete_all_descriptions, delete_description, get_all_descriptions,
    get_description, update_description,
};
use axum::routing::{delete, get, post, put, Route};
use axum::Router;
use common::app_state::AppState;

pub fn get_api_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/descriptions",
            get(get_all_descriptions)
                .post(create_description)
                .delete(delete_all_descriptions),
        )
        .route(
            "/descriptions/:id",
            get(get_description)
                .put(update_description)
                .delete(delete_description),
        )
}
