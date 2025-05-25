use actix_web::{App, HttpServer, Responder, get, middleware::Logger, web};
mod routes;
mod utils;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initializing dotenv (RUST_LOG=info, set in environment)
    dotenv::dotenv().ok();

    // Initializing env_logger
    env_logger::init();

    // Adding logger middleware using `wrap`
    HttpServer::new(|| App::new().wrap(Logger::default()).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
