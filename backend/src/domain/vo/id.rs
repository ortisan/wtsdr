use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> ResultApp<Self> {
        Self::new_from_uuid(Uuid::now_v7())
    }

    pub fn new_from_uuid(id: Uuid) -> ResultApp<Self> {
        Ok(Id(id))
    }

    pub fn new_from_string(id: String) -> ResultApp<Self> {
        let parse_result = Uuid::parse_str(&id);
        match parse_result {
            Ok(uuid) => Ok(Id(uuid)),
            Err(_err) => Err(Arc::new(AppError::Validation(
                ErrorData::new("invalid-uuid", "Invalid UUID")
                    .with_args(HashMap::from([("id".to_string(), id.clone())])),
            ))),
        }
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }
}
