use crate::common::error::{AppError, ErrorData};
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ErrorJsonResponse {
    status_code: u16,
    code: String,
    description: String,
    arguments: Option<HashMap<String, String>>,
}

impl From<AppError> for HttpResponse {
    fn from(value: AppError) -> Self {
        match value {
            AppError::NotFound(ed) => HttpResponse::NotFound()
                .json(get_error_json_response(StatusCode::NOT_FOUND.as_u16(), ed)),
            AppError::IllegalArgument(ed) => HttpResponse::BadRequest().json(
                get_error_json_response(StatusCode::BAD_REQUEST.as_u16(), ed),
            ),
            AppError::Unauthorized(ed) => HttpResponse::Unauthorized().json(
                get_error_json_response(StatusCode::UNAUTHORIZED.as_u16(), ed),
            ),
            AppError::UnprocessableEntity(ed) => HttpResponse::UnprocessableEntity().json(
                get_error_json_response(StatusCode::UNPROCESSABLE_ENTITY.as_u16(), ed),
            ),
            AppError::Database(ed) => HttpResponse::InternalServerError().json(
                get_error_json_response(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), ed),
            ),
            AppError::Validation(ed) => HttpResponse::BadRequest().json(get_error_json_response(
                StatusCode::BAD_REQUEST.as_u16(),
                ed,
            )),
            AppError::Service(ed) => HttpResponse::InternalServerError().json(
                get_error_json_response(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), ed),
            ),
            AppError::Internal(ed) => HttpResponse::InternalServerError().json(
                get_error_json_response(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), ed),
            ),
        }
    }
}

fn get_error_json_response(status_code: u16, error_data: ErrorData) -> ErrorJsonResponse {
    ErrorJsonResponse {
        status_code,
        code: error_data.code,
        description: error_data.message,
        arguments: error_data.args,
    }
}
