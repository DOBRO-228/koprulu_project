// Your app_state module
use handlers::main_page::render_main_page; // Import main_page handler from handlers crate

use common::app_state::init_app_state;

use dotenv::dotenv;

use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let app_state = init_app_state().await;

    // Your application code here
    let app = Router::new()
        .route("/", get(render_main_page)) // Route for home page
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app_state); // Share the app state (db and templates)

    // let app = Router::new()
    //     .route("/", get(main_page))  // Route for home page
    //     .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    //     .with_state(state);  // Share the app state (db and templates)

    // Run the application on port 3000
    let addr: String = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
