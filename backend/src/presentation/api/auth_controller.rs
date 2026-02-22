use actix_web::{HttpResponse, web};
use serde_json::json;

use crate::infrastructure::auth::{
    AuthResponse, LoginRequest, SignupRequest, generate_jwt, hash_password, verify_password,
};
use uuid::Uuid;

/// POST /api/v1/auth/signup
pub async fn signup(body: web::Json<SignupRequest>) -> Result<HttpResponse, actix_web::Error> {
    let password_hash =
        hash_password(&body.password).map_err(actix_web::error::ErrorInternalServerError)?;

    let user_id = Uuid::now_v7();
    let oauth_sub = format!("local|{}", user_id);

    // TODO: persist user to database via repository
    let _user_data = json!({
        "id": user_id,
        "oauth_sub": oauth_sub,
        "name": body.name,
        "email": body.email,
        "password_hash": password_hash,
    });

    let token = generate_jwt(&user_id, &body.name, &body.email)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user_id: user_id.to_string(),
        name: body.name.clone(),
        email: body.email.clone(),
    }))
}

/// POST /api/v1/auth/login
pub async fn login(body: web::Json<LoginRequest>) -> Result<HttpResponse, actix_web::Error> {
    // TODO: fetch user from database by email
    // For now, return mock error if not found
    let mock_password_hash =
        "$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHQ$YhFKjFJEInCAlIJGS1Dl/HKk1E3EOgcvGKs5J7ID8pQ";
    let mock_user_id = Uuid::now_v7();
    let mock_name = "Mock User";

    let valid = verify_password(&body.password, mock_password_hash)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if !valid {
        return Ok(HttpResponse::Unauthorized().json(json!({"error": "Credenciais inválidas"})));
    }

    let token = generate_jwt(&mock_user_id, mock_name, &body.email)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user_id: mock_user_id.to_string(),
        name: mock_name.to_string(),
        email: body.email.clone(),
    }))
}
