use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use validator::ValidationErrors;

pub const INTERNAL_ERROR_CODE: &str = "internal";

#[derive(Debug, Clone)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
    pub args: Option<HashMap<String, String>>,
    pub cause: Option<Arc<dyn StdError>>,
}

impl ErrorData {
    pub fn new<S: Into<String>>(code: S, message: S) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            args: None,
            cause: None,
        }
    }
    pub fn with_args(mut self, args: HashMap<String, String>) -> Self {
        self.args = args.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_cause(mut self, cause: Option<Arc<dyn StdError>>) -> Self {
        self.cause = cause;
        self
    }
}

impl Display for ErrorData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}, Message: {}, Arguments {:?}",
            self.code, self.message, self.args
        )
    }
}

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(ErrorData),
    Internal(ErrorData),
    IllegalArgument(ErrorData),
    Unauthorized(ErrorData),
    UnprocessableEntity(ErrorData),
    Database(ErrorData),
    Validation(ErrorData),
    Service(ErrorData),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(d) => d.fmt(f),
            AppError::Internal(d) => d.fmt(f),
            AppError::IllegalArgument(d) => d.fmt(f),
            AppError::Unauthorized(d) => d.fmt(f),
            AppError::UnprocessableEntity(d) => d.fmt(f),
            AppError::Database(d) => d.fmt(f),
            AppError::Validation(d) => d.fmt(f),
            AppError::Service(d) => d.fmt(f),
        }
    }
}

impl StdError for AppError {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            AppError::NotFound(d) => d.cause.as_ref().map(|c| c.as_ref()),
            AppError::Internal(d) => d.cause.as_ref().map(|c| c.as_ref()),
            AppError::IllegalArgument(d) => d.cause.as_ref().map(|c| c.as_ref()),
            AppError::Unauthorized(d) => d.cause.as_ref().map(|c| c.as_ref()),
            AppError::UnprocessableEntity(d) => d.cause.as_ref().map(|c| c.as_ref()),
            AppError::Database(d) => d.cause.as_ref().map(|c| c.as_ref()),
            AppError::Validation(d) => d.cause.as_ref().map(|c| c.as_ref()),
            AppError::Service(d) => d.cause.as_ref().map(|c| c.as_ref()),
        }
    }
}

impl From<Arc<dyn StdError>> for AppError {
    fn from(value: Arc<dyn StdError>) -> Self {
        if let Some(app_error) = value.downcast_ref::<AppError>() {
            match app_error {
                AppError::NotFound(error_data) => {
                    return AppError::NotFound(error_data.clone());
                }
                AppError::Internal(error_data) => {
                    return AppError::Internal(error_data.clone());
                }
                AppError::IllegalArgument(error_data) => {
                    return AppError::IllegalArgument(error_data.clone());
                }
                AppError::Unauthorized(error_data) => {
                    return AppError::Unauthorized(error_data.clone());
                }
                AppError::UnprocessableEntity(error_data) => {
                    return AppError::UnprocessableEntity(error_data.clone());
                }
                AppError::Database(error_data) => {
                    return AppError::Database(error_data.clone());
                }
                AppError::Validation(error_data) => {
                    return AppError::Validation(error_data.clone());
                }
                AppError::Service(error_data) => {
                    return AppError::Service(error_data.clone());
                }
            }
        }

        AppError::Internal(
            ErrorData::new(INTERNAL_ERROR_CODE, "internal error").with_cause(Some(value)),
        )
    }
}

impl From<ValidationErrors> for AppError {
    fn from(value: ValidationErrors) -> Self {
        AppError::Validation(
            ErrorData::new(INTERNAL_ERROR_CODE, "validation error")
                .with_cause(Some(Arc::new(value.clone()))),
        )
    }
}
