use crate::domain::usecase::user::create_user::{CreateUserUseCase, CreateUserUseCaseImpl};
use crate::domain::usecase::user::delete_user::{DeleteUserUseCase, DeleteUserUseCaseImpl};
use crate::domain::usecase::user::update_user::{UpdateUserUseCase, UpdateUserUseCaseImpl};
use crate::infrastructure::postgres::{DbConfig, PostgresBaseRepository};
use crate::presentation::user::user_route;
use crate::repositories::user::user_repository::{UserRepository, UserRepositoryPostgres};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use std::env;
use std::sync::Arc;
use crate::domain::vo::auth_token::{set_auth_secret};

mod common;
mod domain;
mod infrastructure;
mod presentation;
mod repositories;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let database_url = env::var("DATABASE_URL").unwrap();

    // Put secret key to auth_secret
    set_auth_secret("secret".to_string());

    let db_config = DbConfig { database_url };

    let base_repository = PostgresBaseRepository::new(db_config);
    let user_repository: Arc<dyn UserRepository> =
        Arc::new(UserRepositoryPostgres::new(base_repository));
    let user_repository_data = web::Data::new(user_repository.clone());

    let create_user_use_case: Arc<dyn CreateUserUseCase> =
        Arc::new(CreateUserUseCaseImpl::new(user_repository.clone()));
    let create_user_use_case_data = web::Data::new(create_user_use_case.clone());

    let delete_user_use_case: Arc<dyn DeleteUserUseCase> =
        Arc::new(DeleteUserUseCaseImpl::new(user_repository.clone()));
    let delete_user_use_case_data = web::Data::new(delete_user_use_case.clone());

    let update_user_use_case: Arc<dyn UpdateUserUseCase> =
        Arc::new(UpdateUserUseCaseImpl::new(user_repository.clone()));
    let update_user_use_case_data = web::Data::new(update_user_use_case.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(create_user_use_case_data.clone())
            .app_data(update_user_use_case_data.clone())
            .app_data(delete_user_use_case_data.clone())
            .app_data(user_repository_data.clone())
            .wrap(Logger::default())
            .configure(user_route::routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
