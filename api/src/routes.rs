use super::hormone::{
    create_hormone, delete_all_hormones, delete_hormone, get_all_hormones,
    get_hormone, update_hormone,
};
use axum::routing::get;
use axum::Router;
use common::app_state::AppState;

pub fn get_api_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/hormones",
            get(get_all_hormones)
                .post(create_hormone)
                .delete(delete_all_hormones),
        )
        .route(
            "/hormones/:id",
            get(get_hormone)
                .put(update_hormone)
                .delete(delete_hormone),
        )
}
