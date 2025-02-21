use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalError,
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Not Found")]
    NotFound,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal Server Error".into(),
                })
            }
            AppError::BadRequest(message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: message.clone(),
                })
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "Resource Not Found".into(),
                })
            }
        }
    }
}
