use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use regex::Regex;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> ResultApp<Self> {
        Self::validate(email.clone())?;
        Ok(Self(email))
    }

    pub fn validate(value: String) -> ResultApp<()> {
        let re = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@([a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,}$").unwrap();
        if re.is_match(value.trim()) {
            return Ok(());
        }
        Err(Arc::new(AppError::Validation(ErrorData::new(
            "invalid-email",
            "Invalid email address",
        ))))
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email_is_accepted() {
        let e = Email::new("user@example.com".to_string());
        assert!(e.is_ok());
        assert_eq!(e.unwrap().value(), "user@example.com".to_string());
    }

    #[test]
    fn invalid_email_is_rejected_with_validation_code() {
        let err = Email::new("bad@".to_string()).unwrap_err();
        if let Some(app_err) = err.downcast_ref::<crate::common::error::AppError>() {
            match app_err {
                crate::common::error::AppError::Validation(data) => {
                    assert_eq!(data.code, "invalid-email");
                }
                other => panic!("expected Validation error, got: {:?}", other),
            }
        } else {
            panic!("error is not AppError");
        }
    }
}
