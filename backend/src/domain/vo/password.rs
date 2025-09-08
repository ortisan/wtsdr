use crate::common::result::ResultApp;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(value: String) -> ResultApp<Self> {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        let result = hasher.finalize();
        let hex_hash_password = hex::encode(result);
        Ok(Self(hex_hash_password))
    }

    pub fn new_from_hashed_value(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}
