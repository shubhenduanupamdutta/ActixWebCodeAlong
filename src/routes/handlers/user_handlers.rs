use actix_web::{get, web};
use sea_orm::EntityTrait;

use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};

#[get("")]
pub async fn user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, e.to_string()))?
        .ok_or(ApiResponse::new(401, String::from("User not found.")))?;

    Ok(ApiResponse::new(
        200,
        format!(
            "Verified User, {{ 'name': {}, 'email': {} }}",
            user_model.name, user_model.email
        ),
    ))
}
