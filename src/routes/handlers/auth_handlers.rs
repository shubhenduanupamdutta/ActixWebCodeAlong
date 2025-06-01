use actix_web::{Responder, post, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::utils::{api_response, app_state};

#[derive(Serialize, Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginModel {
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
        password: Set(secure_hash(register_json.password.clone())),
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

#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> impl Responder {
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(&login_json.email))
        .one(&app_state.db)
        .await
        .unwrap();

    if user.is_none() {
        return api_response::ApiResponse::new(401, "User not found".to_string());
    }

    let user = user.unwrap();
    match verify_hash(login_json.password.clone(), user.password) {
        false => api_response::ApiResponse::new(401, "Wrong Password".to_string()),
        true => api_response::ApiResponse::new(200, user.name),
    }
}

/// Securely hash the text using Argon2 default methods and return the PHC String of the new hash
pub(crate) fn secure_hash(text: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(&text.into_bytes(), &salt)
        .unwrap()
        .to_string()
}

/// Verify the text corresponds to the hash. If it corresponds then return `true` otherwise `false`
pub(crate) fn verify_hash(text: String, hash: String) -> bool {
    let parsed_hash = match PasswordHash::new(&hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(&text.into_bytes(), &parsed_hash)
        .is_ok()
}
