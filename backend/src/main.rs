use crate::config::Config;
use crate::infrastructure::state::AppState;
use crate::presentation::api::auth_controller::{login, signup};
use crate::presentation::api::broadcast_controller::broadcast_request;
use crate::presentation::api::services_controller::{get_service, list_services};
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "WTSDR API",
        version = "0.1.0",
        description = "Marketplace de serviços locais"
    ),
    paths(),
    tags(
        (name = "auth", description = "Autenticação (signup/login)"),
        (name = "services", description = "Serviços e provedores"),
        (name = "broadcast", description = "Broadcast de solicitações de serviço"),
        (name = "health", description = "Health check")
    )
)]
struct ApiDoc;

#[get("/api/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/signup", web::post().to(signup))
                    .route("/login", web::post().to(login)),
            )
            .service(
                web::scope("/services")
                    .route("", web::get().to(list_services))
                    .route("/{id}", web::get().to(get_service)),
            )
            .route("/broadcast", web::post().to(broadcast_request)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::from_env();
    info!("Starting server at {}", config.server_bind);

    let state = AppState::new(config).expect("Failed to initialize AppState");
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .service(health_check)
            .configure(configure_app)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
