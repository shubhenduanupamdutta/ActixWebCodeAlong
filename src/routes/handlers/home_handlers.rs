use actix_web::{Responder, get, web};
use sea_orm::{ConnectionTrait, Statement};

use crate::utils::{api_response::ApiResponse, app_state::AppState};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    ApiResponse::new(200, format!("Hello {name}"))
}

#[get("/test")]
pub async fn test(app_state: web::Data<AppState>) -> impl Responder {
    let _res = app_state
        .db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "select * from user;",
        ))
        .await
        .unwrap();
    ApiResponse::new(200, "Test Call".to_string())
}
