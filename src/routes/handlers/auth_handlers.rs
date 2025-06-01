use actix_web::{Responder, post, web};
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response, app_state};

#[derive(Serialize, Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>,
) -> impl Responder {
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(register_json.password.clone()),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(
        201,
        format!("User is registered with id: {}", user_model.id),
    )
}
