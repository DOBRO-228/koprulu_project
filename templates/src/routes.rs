use crate::handlers::contacts::render_contacts_page;
use crate::handlers::knowledge_base::render_knowledge_base_page;
use crate::handlers::main_page::render_main_page;
use axum::routing::get;
use axum::Router;
use common::app_state::AppState;

pub fn get_template_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(render_main_page))
        .route("/knowledge_base", get(render_knowledge_base_page))
        .route("/contacts", get(render_contacts_page))
}
