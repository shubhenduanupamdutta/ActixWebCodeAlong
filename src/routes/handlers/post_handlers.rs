use std::path::PathBuf;

use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web};
use chrono::{FixedOffset, Utc};
use entity::post;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait, TryIntoModel,
};
use uuid::Uuid;

use crate::{
    schemas::{
        post_schemas::{CreatePostModel, PostOut},
        user_schemas::UserOut,
    },
    utils::{api_response::ApiResponse, app_state, constants::get_max_file_size, jwt::Claims},
};

#[post("create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: MultipartForm<CreatePostModel>,
) -> Result<ApiResponse, ApiResponse> {
    let mut post_out;

    let id = claim.id;
    if post_model.file.is_none() {
        post_out = create_post_without_image(app_state.clone(), claim, post_model).await?;
    } else {
        post_out = create_post_with_image(app_state.clone(), claim, post_model).await?;
    }

    let user = entity::user::Entity::find_by_id(id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    post_out.user = user.map(|model| UserOut {
        id: model.id,
        name: model.name,
        email: model.email,
    });

    ApiResponse::serialize(201, &post_out)
}

async fn create_post_with_image(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: MultipartForm<CreatePostModel>,
) -> Result<PostOut, ApiResponse> {
    let in_file = post_model.file.as_ref().unwrap();
    let check_name = in_file.file_name.clone().unwrap_or("null".to_owned());

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

    match in_file.size {
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

    let tmp_file_path = in_file.file.path();
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
    }

    let new_post = updated_post.try_into_model().map_err(|err| {
        ApiResponse::new(
            201,
            format!(
                "Post created, but unable to return details. Error details: {}",
                err
            ),
        )
    })?;

    Ok(PostOut::from(new_post))
}

async fn create_post_without_image(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: MultipartForm<CreatePostModel>,
) -> Result<PostOut, ApiResponse> {
    let new_post = post::ActiveModel {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(PostOut::from(new_post))
}

#[get("my-posts")]
pub async fn get_my_posts(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let posts: Vec<PostOut> = entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(claim.id))
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(PostOut::from)
        .collect();

    ApiResponse::serialize(200, &posts)
}

#[get("all-posts")]
pub async fn get_all_posts(
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let posts: Vec<PostOut> = entity::post::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(PostOut::from)
        .collect();

    ApiResponse::serialize(200, &posts)
}

#[get("{post_uuid}")]
pub async fn get_one_post(
    app_state: web::Data<app_state::AppState>,
    post_uuid: web::Path<Uuid>,
) -> Result<ApiResponse, ApiResponse> {
    let post: PostOut = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(*post_uuid))
        .find_also_related(entity::user::Entity)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .map(|post| {
            let mut post_out = PostOut::from(post.0);
            post_out.user = post.1.map(|model| UserOut {
                id: model.id,
                name: model.name,
                email: model.email,
            });
            post_out
        })
        .ok_or(ApiResponse::new(404, "No post found".to_string()))?;

    ApiResponse::serialize(200, &post)
}
