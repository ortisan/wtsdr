use crate::common::result::ResultApp;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::sync::Arc;
use std::sync::OnceLock;

pub static AUTH_SECRET: OnceLock<String> = OnceLock::new();

fn set_auth_secret(secret: String) -> () {
    AUTH_SECRET.set(secret).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}

#[derive(Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub claims: Claims,
}

impl AuthToken {
    pub fn new(token: String) -> ResultApp<Self> {
        let token_decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        ).map_err(|e| Arc::new(e) as Arc<dyn StdError>)?;
        Ok(Self {
            token,
            claims: token_decoded.claims,
        })
    }
}
