use handlers::knowledge_base::render_knowledge_base_page;
use handlers::main_page::render_main_page;

use common::app_state::init_app_state;

use dotenv::dotenv;

use api::description::{
    create_description, delete_all_descriptions, delete_description, get_all_descriptions,
    get_description, update_description,
};
use axum::routing::{delete, post, put};
use axum::{routing::get, Router};
use handlers::contacts::render_contacts_page;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let app_state = init_app_state().await;
    let static_files = ServeDir::new("./static");

    let app = Router::new()
        .nest_service(app_state.routes.static_files, static_files)
        .route(
            app_state.routes.get_all_descriptions,
            get(get_all_descriptions),
        )
        .route(app_state.routes.get_description, get(get_description))
        .route(
            app_state.routes.create_description,
            post(create_description),
        )
        .route(app_state.routes.update_description, put(update_description))
        .route(
            app_state.routes.delete_description,
            delete(delete_description),
        )
        .route(
            app_state.routes.delete_all_descriptions,
            delete(delete_all_descriptions),
        )
        .route(app_state.routes.main_page, get(render_main_page))
        .route(
            app_state.routes.knowledge_base,
            get(render_knowledge_base_page),
        )
        .route(app_state.routes.contacts, get(render_contacts_page))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app_state);

    let addr: String = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
