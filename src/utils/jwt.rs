use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use super::constants;

/// Details in JSON Web Token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Expiration time as UTC timestamp
    pub exp: usize,
    /// Issued at as UTC timestamp
    pub iat: usize,
    /// Email of the user
    pub email: String,
    /// Id of the user
    pub id: i32,
}

/// Encode provided email and id with necessary details and return a JWT.
pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let duration = Duration::hours(24);

    let claims = Claims {
        exp: (now + duration).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };

    let secret = constants::get_secret().clone();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

/// Decode a JWT and return data as Claims
pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = constants::get_secret().clone();
    decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}
