use entity::user;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserUpdate {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LoginUser {
    pub email: String,
    pub password: String,
}

pub type UserOut = UserUpdate;

impl From<user::Model> for UserUpdate {
    fn from(value: user::Model) -> Self {
        UserUpdate {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}
