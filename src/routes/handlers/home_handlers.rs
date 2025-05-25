use actix_web::{Responder, get, web};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/test")]
pub async fn test() -> impl Responder {
    "Test call".to_string()
}
