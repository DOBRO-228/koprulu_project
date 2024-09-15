use common::app_state::init_app_state;

use dotenv::dotenv;

use api::routes::get_api_routes;
use axum::Router;
use templates::routes::get_template_routes;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let app_state = init_app_state().await;
    let static_files = ServeDir::new("./static");

    let api_routes = get_api_routes();
    let template_routes = get_template_routes();
    let app = Router::new()
        .nest("/api", api_routes)
        .merge(template_routes)
        .nest_service("/static", static_files)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app_state);

    let addr: String = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
