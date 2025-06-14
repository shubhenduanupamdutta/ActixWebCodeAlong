use actix_web::{get, put, web};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUserModel {
    name: String,
}

#[get("")]
pub async fn user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, e.to_string()))?
        .ok_or(ApiResponse::new(404, String::from("User not found.")))?;

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'message': 'Verified User', 'name': '{}', 'email': '{}' }}",
            user_model.name, user_model.email
        ),
    ))
}

#[put("update")]
pub async fn update_user(
    app_state: web::Data<app_state::AppState>,
    user_data: web::Json<UpdateUserModel>,
    claim: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let mut user_model = entity::user::Entity::find_by_id(claim.id)
        .one(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, e.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_string()))?
        .into_active_model();

    user_model.name = Set(user_data.name.clone());
    let user_model = user_model
        .update(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(
        200,
        format!(
            "{{ 'message': 'User Updated', 'name': '{}', 'email': '{}' }}",
            user_model.name, user_model.email
        ),
    ))
}
