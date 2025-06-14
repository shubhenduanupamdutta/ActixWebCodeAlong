use actix_web::{get, post, web};
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};

#[derive(Debug, Serialize, Deserialize)]
struct CreatePostModel {
    title: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostModel {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: Option<String>,
    pub user_id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[post("create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: web::Json<CreatePostModel>,
) -> Result<ApiResponse, ApiResponse> {
    let post_entity = entity::post::ActiveModel {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())),
        ..Default::default()
    };

    let model = post_entity
        .insert(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(
        201,
        format!(
            "{{ 'message': 'Post created', 'id': '{}', 'title': '{}' }}",
            model.id, model.text
        ),
    ))
}

#[get("my-posts")]
pub async fn get_my_posts(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(claim.id))
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
        .collect();

    let res_str =
        serde_json::to_string(&posts).map_err(|err| ApiResponse::new(500, err.to_string()))?;
    Ok(ApiResponse::new(200, res_str))
}
