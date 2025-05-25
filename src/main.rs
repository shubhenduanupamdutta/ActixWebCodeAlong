use actix_web::{App, HttpServer, middleware::Logger};
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

    // Adding logger middleware using `wrap`
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
