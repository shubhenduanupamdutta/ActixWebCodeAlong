//! Proper response structure

use std::fmt::Display;

use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web, HttpResponse, Responder, ResponseError,
};
use serde_json::json;

use crate::error::MainError;

#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap_or_default(),
        }
    }

    pub fn json(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,
            body: json!({"message": body}).to_string(),
            response_code: StatusCode::from_u16(status_code).unwrap_or_default(),
        }
    }

    pub fn serialize<T: serde::Serialize>(status_code: u16, data: &T) -> Result<Self, Self> {
        let body =
            serde_json::to_string(data).map_err(|err| ApiResponse::new(500, err.to_string()))?;
        Ok(ApiResponse {
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap_or_default(),
        })
    }
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}, \nStatus Code: {}",
            self.body, self.status_code
        )
    }
}

#[allow(unused_variables)]
impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::build(self.response_code)
            .insert_header(ContentType::json())
            .body(body)
    }
}

impl ResponseError for ApiResponse {
    fn status_code(&self) -> StatusCode {
        self.response_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let json_body = serde_json::to_string(&MainError {
            message: self.body.clone(),
        })
        .unwrap_or_else(|_| r#"{"error": "Error converting to json"}"#.to_string());
        let body = BoxBody::new(web::BytesMut::from(json_body.as_bytes()));
        HttpResponse::build(self.response_code)
            .insert_header(ContentType::json())
            .body(body)
    }
}
