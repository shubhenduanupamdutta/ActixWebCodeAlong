use actix_web::{Responder, get, web};

use crate::utils::{api_response, app_state};

#[allow(unused_variables)]
#[get("")]
pub async fn user(app_state: web::Data<app_state::AppState>) -> impl Responder {
    api_response::ApiResponse::new(200, "Verified User".to_string())
}
