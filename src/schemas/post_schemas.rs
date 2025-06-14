use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use chrono::{DateTime, FixedOffset};
use entity::post;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schemas::user_schemas::UserOut;

#[derive(Debug, MultipartForm)]
pub struct CreatePostModel {
    pub title: Text<String>,
    pub text: Text<String>,
    pub file: Option<TempFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOut {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: Option<String>,
    pub user_id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub user: Option<UserOut>,
}

impl From<post::Model> for PostOut {
    fn from(value: post::Model) -> Self {
        PostOut {
            id: value.id,
            title: value.title,
            text: value.text,
            uuid: value.uuid,
            image: value.image,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            user: None,
        }
    }
}
