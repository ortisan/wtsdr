use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: String) -> ResultApp<Self> {
        let s = s.trim();
        if s.len() > 2048 {
            return Err(Arc::new(AppError::Validation(ErrorData::new(
                "invalid-description",
                "Invalid description.",
            ))));
        }
        Ok(Description(s.to_owned()))
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
}
