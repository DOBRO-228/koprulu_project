use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tera::Tera;
use crate::routes::Routes;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub templates: Arc<Tera>,
    pub routes: Arc<Routes>,
}

pub async fn init_app_state() -> AppState {
    let db = setup_db().await;
    let templates = setup_template_rendering();
    let routes = Routes::default();

    AppState {
        db: Arc::new(db),
        templates: Arc::new(templates),
        routes: Arc::new(routes),
    }
}

async fn setup_db() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .connect_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(60))
        .sqlx_logging(true);

    Database::connect(opt).await.unwrap()
}

fn setup_template_rendering() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}
