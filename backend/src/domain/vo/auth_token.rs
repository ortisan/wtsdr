use crate::common::result::ResultApp;
use jsonwebtoken::{decode, DecodingKey, Validation, encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::sync::Arc;
use std::sync::OnceLock;
use chrono::Utc;
use crate::domain::entity::user::User;

static AUTH_SECRET: OnceLock<String> = OnceLock::new();

pub fn set_auth_secret(secret: String) -> () {
    AUTH_SECRET.set(secret).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
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

    pub fn new(user: User) -> Self {
        let expiration = Utc::now() + chrono::Duration::days(1);

        let claims = Claims {
            sub: user.id.value(),
            exp: expiration.timestamp() as usize
        };

        let secret = AUTH_SECRET.get().expect("AUTH_SECRET not set");
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
            .expect("failed to encode token");
        Self { token, claims }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::vo::{id::Id, name::Name, email::Email, password::Password, temporal::DateTime};
    use crate::domain::entity::user::User;
    use std::sync::Once;
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use chrono::Utc;

    static INIT: Once = Once::new();

    fn init_secret_once() {
        INIT.call_once(|| {
            set_auth_secret("test-secret-123".to_string());
        });
    }

    fn build_user() -> User {
        let id = Id::new().unwrap();
        let name = Name::new("John Doe").unwrap();
        let email = Email::new("john.doe@example.com".to_string()).unwrap();
        let password = Some(Password::new("s3cr3t".to_string()).unwrap());
        let created_at = DateTime::new();
        let updated_at = DateTime::new();

        User::new(
            id,
            name,
            email,
            password,
            None,
            false,
            created_at,
            updated_at,
            None,
        )
    }

    #[test]
    fn test_generate_token_and_decode() {
        init_secret_once();
        let user = build_user();
        let token = AuthToken::new(user.clone());

        let decoded = decode::<super::Claims>(
            &token.token,
            &DecodingKey::from_secret(AUTH_SECRET.get().unwrap().as_bytes()),
            &Validation::default(),
        ).expect("token should decode");

        assert_eq!(decoded.claims.sub, user.id.value());
        let now = Utc::now().timestamp() as usize;
        assert!(decoded.claims.exp > now, "exp should be in the future");
    }

    #[test]
    fn test_parse_returns_ok() {
        init_secret_once();
        let user = build_user();
        let token = AuthToken::new(user.clone());
        let parsed = AuthToken::parse(token.token.clone());
        assert!(parsed.is_ok(), "parse should succeed for a valid token");
        let parsed = parsed.unwrap();
        assert_eq!(parsed.token, token.token);

        let decoded = decode::<super::Claims>(
            &parsed.token,
            &DecodingKey::from_secret(AUTH_SECRET.get().unwrap().as_bytes()),
            &Validation::default(),
        ).expect("token should decode");
        assert_eq!(decoded.claims.sub, user.id.value());
    }

    #[test]
    fn test_parse_fails_on_tampered_token() {
        init_secret_once();
        let user = build_user();
        let token = AuthToken::new(user);

        let mut tampered = token.token.clone();
        if let Some(ch) = tampered.chars().find(|c| *c != '.') {
            let pos = tampered.find(ch).unwrap();
            tampered.replace_range(pos..pos+1, if ch != 'A' { "A" } else { "B" });
        } else {
            tampered.push('A');
        }

        let parsed = AuthToken::parse(tampered);
        assert!(parsed.is_err(), "parse should fail for tampered token");
    }
}
