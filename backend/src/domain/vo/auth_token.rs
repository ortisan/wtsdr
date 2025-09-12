use crate::common::result::ResultApp;
use jsonwebtoken::{decode, DecodingKey, Validation, encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::sync::Arc;
use std::sync::OnceLock;

static AUTH_SECRET: OnceLock<String> = OnceLock::new();

pub fn set_auth_secret(secret: String) -> () {
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
    pub fn parse(token: String) -> ResultApp<Self> {
        let token_decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(AUTH_SECRET.get().unwrap().as_ref()),
            &Validation::default(),
        ).map_err(|e| Arc::new(e) as Arc<dyn StdError>)?;
        Ok(Self {
            token,
            claims: token_decoded.claims,
        })
    }

    pub fn new(claims: Claims) -> Self {
        let secret = AUTH_SECRET.get().expect("AUTH_SECRET not set");
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
            .expect("failed to encode token");
        Self { token, claims }
    }
}

mod test {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::time::{SystemTime, UNIX_EPOCH, Duration};

    #[test]
    fn generate_token_and_validate_claims() {
        // Arrange
        let secret = "test-secret-123".to_string();
        if super::AUTH_SECRET.get().is_none() {
            set_auth_secret(secret.clone());
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let exp = (now + Duration::from_secs(3600)).as_secs();

        let claims = Claims {
            aud: "wtsdr-app".to_string(),
            sub: "user-123".to_string(),
            company: "Acme Corp".to_string(),
            exp,
        };

        // Act - create a new token from claims
        let created = AuthToken::new(claims.clone());

        // Parse it back to ensure it's valid and signed with the same secret
        let parsed = AuthToken::parse(created.token.clone()).expect("failed to parse token");

        // Assert
        assert_eq!(created.claims.aud, claims.aud);
        assert_eq!(created.claims.sub, claims.sub);
        assert_eq!(created.claims.company, claims.company);
        assert_eq!(created.claims.exp, claims.exp);

        assert_eq!(parsed.token, created.token);
        assert_eq!(parsed.claims.aud, claims.aud);
        assert_eq!(parsed.claims.sub, claims.sub);
        assert_eq!(parsed.claims.company, claims.company);
        assert_eq!(parsed.claims.exp, claims.exp);
    }

    #[test]
    fn generate_token_and_validate_claims_using_external_encode() {
        // Arrange
        let secret = "test-secret-123".to_string();
        // Set secret only once; if already set (e.g., in another test), skip setting to avoid panic.
        if super::AUTH_SECRET.get().is_none() {
            set_auth_secret(secret.clone());
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let exp = (now + Duration::from_secs(3600)).as_secs();

        let claims = Claims {
            aud: "wtsdr-app".to_string(),
            sub: "user-123".to_string(),
            company: "Acme Corp".to_string(),
            exp,
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).expect("failed to encode token");

        // Act
        let parsed = AuthToken::parse(token.clone()).expect("failed to parse token");

        // Assert
        assert_eq!(parsed.token, token);
        assert_eq!(parsed.claims.aud, "wtsdr-app");
        assert_eq!(parsed.claims.sub, "user-123");
        assert_eq!(parsed.claims.company, "Acme Corp");
        assert_eq!(parsed.claims.exp, exp);
    }
}
