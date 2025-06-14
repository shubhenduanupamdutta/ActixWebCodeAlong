use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{get, post, web::{self, post}};
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};

#[derive(Debug, MultipartForm)]
struct CreatePostModel {
    title: Text<String>,
    text: Text<String>,
    file: TempFile,
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
    pub user: Option<UserModel>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserModel {
    name: String,
    email: String,
}

#[post("create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: MultipartForm<CreatePostModel>,
) -> Result<ApiResponse, ApiResponse> {

    let check_name = post_model.file.file_name.clone().unwrap_or("null".to_owned());

    match &check_name[check_name.len() -4..] {
        ".png" | ".jpg" => (),
        _ => return Err(ApiResponse::new(401, "Bad Request, Invalid File Name".to_string())) 
    }

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
            user: None,
        })
        .collect();

    let res_str =
        serde_json::to_string(&posts).map_err(|err| ApiResponse::new(500, err.to_string()))?;
    Ok(ApiResponse::new(200, res_str))
}

#[get("all-posts")]
pub async fn get_all_posts(
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
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
            user: None,
        })
        .collect();

    let res_str =
        serde_json::to_string(&posts).map_err(|err| ApiResponse::new(500, err.to_string()))?;
    Ok(ApiResponse::new(200, res_str))
}

#[get("{post_uuid}")]
pub async fn get_one_post(
    app_state: web::Data<app_state::AppState>,
    post_uuid: web::Path<Uuid>,
) -> Result<ApiResponse, ApiResponse> {
    let posts: PostModel = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(*post_uuid))
        .find_also_related(entity::user::Entity)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .map(|post| PostModel {
            id: post.0.id,
            title: post.0.title,
            text: post.0.text,
            uuid: post.0.uuid,
            image: post.0.image,
            user_id: post.0.user_id,
            created_at: post.0.created_at,
            updated_at: post.0.updated_at,
            user: post.1.map(|model| UserModel {
                name: model.name,
                email: model.email,
            }),
        })
        .ok_or(ApiResponse::new(404, "No post found".to_string()))?;

    let res_str =
        serde_json::to_string(&posts).map_err(|err| ApiResponse::new(500, err.to_string()))?;
    Ok(ApiResponse::new(200, res_str))
}
