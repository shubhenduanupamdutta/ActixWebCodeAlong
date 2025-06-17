use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
}
