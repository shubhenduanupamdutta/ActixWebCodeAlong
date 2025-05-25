use actix_web::{Responder, get, web};

use crate::utils::api_response::ApiResponse;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    ApiResponse::new(200, format!("Hello {name}"))
}

#[get("/test")]
pub async fn test() -> impl Responder {
    ApiResponse::new(200, "Test Call".to_string())
}
