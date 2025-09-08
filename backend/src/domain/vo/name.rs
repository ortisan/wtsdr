use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn new<S: AsRef<str>>(s: S) -> ResultApp<Self> {
        let s = s.as_ref().trim();
        if s.is_empty() || s.len() > 120 {
            return Err(Arc::new(AppError::Validation(ErrorData::new(
                "invalid-field",
                "name must be 1..=120 characters",
            ))));
        }
        Ok(Name(s.to_owned()))
    }
    pub fn value(&self) -> String {
        String::from(&self.0)
    }
}
