
#[derive(Debug, Clone)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Clone)]
pub struct LoginResponse {
    token: String,
}