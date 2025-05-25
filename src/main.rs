use actix_web::{App, HttpServer, middleware::Logger, web};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;
mod routes;
mod utils;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initializing dotenv (RUST_LOG=info, set in environment)
    dotenv::dotenv().ok();

    // Initializing env_logger
    env_logger::init();

    // Getting address and port from env file using OnceLock
    let address = utils::constants::get_address().clone();
    let port = *utils::constants::get_port();
    let db_url = utils::constants::db_url().clone();
    // Database Connection
    let db: DatabaseConnection = Database::connect(db_url).await.unwrap();
    // Running new migration at startup
    Migrator::up(&db, None).await.unwrap();
    
    // App state to use db connection to across all routes
    // Adding logger middleware using `wrap`
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
