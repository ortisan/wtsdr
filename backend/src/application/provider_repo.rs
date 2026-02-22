use crate::domain::{
    error::AppError,
    provider_profile::{Location, ProviderProfile},
};
use async_trait::async_trait;

#[async_trait]
pub trait ProviderRepository: Send + Sync {
    async fn find_within_radius(
        &self,
        target: &Location,
        radius_meters: f64,
        service_type: &str,
    ) -> Result<Vec<ProviderProfile>, AppError>;

    async fn create_provider(&self, provider: &ProviderProfile) -> Result<(), AppError>;
}
