use actix_web::{post, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::{
    error::MainError,
    schemas::{
        token_schema::TokenResponse,
        user_schemas::{LoginUser, User, UserOut},
    },
    utils::{api_response::ApiResponse, app_state, jwt::encode_jwt},
};

#[post("/register")]
pub(crate) async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<User>,
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
pub(crate) async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginUser>,
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
