#[macro_use] extern crate rocket;
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use api::description::create_description_handler;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    env_logger::init();
    let db = setup_db().await;

    // Run the application on port 3000
    let addr: String = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on {}", addr);
    rocket::build().mount("/", routes![index])
}

async fn setup_db() -> DatabaseConnection {
    let database_url = format!(
        "postgres://{user}:{pass}@localhost:{port}/{db}",
        user = env::var("POSTGRES_USER").unwrap(),
        pass = env::var("POSTGRES_PASSWORD").unwrap(),
        port = env::var("POSTGRES_PORT").unwrap(),
        db = env::var("POSTGRES_DB").unwrap(),
    );
    let mut opt = ConnectOptions::new("protocol://username:password@host/database");
    opt.max_connections(100)
        .connect_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(60))
        .sqlx_logging(true);
    Database::connect(&database_url).await.unwrap()
}
