use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    middleware::Next,
};
use jsonwebtoken::TokenData;

use crate::utils::{
    api_response::{self, ApiResponse},
    jwt::{Claims, decode_jwt},
};

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get(header::AUTHORIZATION);

    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        )));
    }

    let token: String = auth
        .ok_or(Error::from(ApiResponse::new(
            400,
            "Bad Request".to_string(),
        )))?
        .to_str()
        .map_err(|err| Error::from(ApiResponse::new(400, err.to_string())))?
        .replace("Bearer ", "")
        .to_owned();
    let claim: TokenData<Claims> =
        decode_jwt(token).map_err(|err| ApiResponse::new(400, err.to_string()))?;
    req.extensions_mut().insert(claim.claims);

    next.call(req)
        .await
        .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}
