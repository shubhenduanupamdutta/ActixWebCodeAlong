use actix_web::{post, web};
use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::{
    error::MainError,
    utils::{api_response::ApiResponse, app_state, jwt::encode_jwt},
};

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

#[derive(Debug, Serialize, Deserialize)]
struct UserOut {
    id: i32,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
}

impl From<user::Model> for UserOut {
    fn from(value: user::Model) -> Self {
        UserOut {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>,
) -> Result<ApiResponse, ApiResponse> {
    let hash = secure_hash(register_json.password.clone())
        .map_err(|err| ApiResponse::new(500, format!("Hashing Failed. Details: {}", err)))?;

    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(hash),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    ApiResponse::serialize(201, &UserOut::from(user_model))
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> Result<ApiResponse, ApiResponse> {
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(&login_json.email))
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_string()))?;

    verify_hash(login_json.password.clone(), &user.password)
        .map_err(|err| ApiResponse::new(401, format!("Unauthorized. Wrong Password. {}", err)))?;

    let jwt =
        encode_jwt(user.email, user.id).map_err(|err| ApiResponse::new(500, err.to_string()))?;

    ApiResponse::serialize(
        200,
        &TokenResponse {
            access_token: jwt,
            token_type: "bearer".to_string(),
        },
    )
}

/// Securely hash the text using Argon2 default methods and return the PHC String of the new hash
pub(crate) fn secure_hash(text: String) -> Result<String, MainError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(&text.into_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| MainError {
            message: err.to_string(),
        })
}

/// Verify the text corresponds to the hash. If it corresponds then return `true` otherwise `false`
pub(crate) fn verify_hash(text: String, hash: &str) -> Result<(), MainError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|err| MainError {
        message: err.to_string(),
    })?;

    Argon2::default()
        .verify_password(&text.into_bytes(), &parsed_hash)
        .map_err(|err| MainError {
            message: err.to_string(),
        })
}
