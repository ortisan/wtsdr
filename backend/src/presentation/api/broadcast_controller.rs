use crate::application::usecases::broadcast::BroadcastServiceRequestUseCase;
use crate::domain::request::ServiceRequest;
use crate::infrastructure::{db::postgis_repo::DieselProviderRepository, state::AppState};
use actix_web::{HttpResponse, web};

// Example Controller Route
pub async fn broadcast_request(
    state: web::Data<AppState>,
    req_body: web::Json<ServiceRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let repo = DieselProviderRepository::new(state.db_pool.clone());
    let usecase = BroadcastServiceRequestUseCase {
        provider_repo: &repo,
    };

    let providers = usecase
        .execute(&req_body.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(providers))
}
