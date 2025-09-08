use actix_web::{dev::RequestHead, guard::Guard, HttpRequest};
use actix_web::guard::GuardContext;

pub struct AuthGuard;

impl Guard for AuthGuard {
    fn check(&self, guard_context: &GuardContext) -> bool {
        // if let Some(auth_header) = guard_context.headers().get("Authorization") {
        //     if let Ok(value) = auth_header.to_str() {
        //         if let Some(token) = value.strip_prefix("Bearer ") {
        //             return validate_token(token);
        //         }
        //     }
        // }
        false
    }
}

fn validate_token(token: &str) -> bool {
    token == "valid_token"
}