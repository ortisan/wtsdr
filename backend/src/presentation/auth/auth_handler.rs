use actix_web::{HttpResponse, get};
use casdoor_rust_sdk::{AuthService, CasdoorConfig};

pub fn abs_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let absolute_path = std::env::current_dir()?.join(path);
    Ok(absolute_path.to_str().unwrap().to_string())
}

#[get("/signup")]
pub async fn signup() -> HttpResponse {
    let conf = CasdoorConfig::from_toml(abs_path("conf.toml").unwrap().as_str()).unwrap();
    let auth_service = AuthService::new(&conf);
    let redirect_url = auth_service.get_signup_url_enable_password();
    HttpResponse::Created().json(redirect_url)
}
