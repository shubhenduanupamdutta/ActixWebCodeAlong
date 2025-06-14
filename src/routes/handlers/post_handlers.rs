use std::path::PathBuf;

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{get, post, web};
use chrono::{DateTime, FixedOffset, Utc};
use entity::post;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait, TryIntoModel,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{
    api_response::ApiResponse, app_state, constants::get_max_file_size, jwt::Claims,
};

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
    let check_name = post_model
        .file
        .file_name
        .clone()
        .unwrap_or("null".to_owned());
    let max_file_size = get_max_file_size() as usize;

    match &check_name[check_name.len() - 4..] {
        ".png" | ".jpg" => (),
        _ => {
            return Err(ApiResponse::new(
                400,
                "Bad Request, Invalid File Name".to_string(),
            ));
        }
    }

    match post_model.file.size {
        0 => return Err(ApiResponse::new(400, "Invalid File Type".to_string())),
        length if length > max_file_size => {
            return Err(ApiResponse::new(400, "File too big".to_string()));
        }
        _ => (),
    }

    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let mut post_entity: post::ActiveModel = post::ActiveModel {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())),
        ..Default::default()
    };

    let tmp_file_path = post_model.file.file.path();
    let file_name = check_name.as_str();

    let time_stamp: i64 = Utc::now().timestamp();
    let mut file_path = PathBuf::from("./public");
    let new_file_name = format!("{}-{}", time_stamp, file_name);
    file_path.push(new_file_name.clone());

    let updated_post;
    match std::fs::copy(tmp_file_path, file_path) {
        Ok(_) => {
            std::fs::remove_file(tmp_file_path).unwrap_or_default();
            post_entity.image = Set(Some(new_file_name.clone()));
            updated_post = post_entity
                .save(&txn)
                .await
                .map_err(|err| ApiResponse::new(500, err.to_string()))?;

            txn.commit()
                .await
                .map_err(|err| ApiResponse::new(500, err.to_string()))?;
        }
        Err(e) => {
            std::fs::remove_file(tmp_file_path).unwrap_or_default();
            txn.rollback()
                .await
                .map_err(|err| ApiResponse::new(500, format!("Errors: {} and {}", e, err)))?;
            return Err(ApiResponse::new(
                500,
                format!("Internal server error. Details: {}", e),
            ));
        }
    };

    let new_post = updated_post.try_into_model().map_err(|err| {
        ApiResponse::new(
            201,
            format!(
                "Post created, but unable to return details. Error details: {}",
                err.to_string()
            ),
        )
    })?;

    let user = entity::user::Entity::find_by_id(claim.id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User Not found.".to_string()))?;

    let post_details = PostModel {
        id: new_post.id,
        title: new_post.title,
        text: new_post.text,
        uuid: new_post.uuid,
        image: new_post.image,
        user_id: new_post.user_id,
        created_at: new_post.created_at,
        updated_at: new_post.updated_at,
        user: Some(UserModel {
            name: user.name,
            email: user.email,
        }),
    };

    let res_str = serde_json::to_string(&post_details)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(201, res_str))
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
