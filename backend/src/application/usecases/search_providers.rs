use crate::application::provider_repo::ProviderRepository;
use crate::domain::error::AppError;
use crate::domain::provider_profile::Location;
use crate::domain::provider_profile::ProviderProfile;

pub struct SearchProvidersUseCase<'a, R: ProviderRepository> {
    pub provider_repo: &'a R,
}

impl<'a, R: ProviderRepository> SearchProvidersUseCase<'a, R> {
    pub async fn execute(
        &self,
        target_location: &Location,
        radius_meters: f64,
        service_type: &str,
    ) -> Result<Vec<ProviderProfile>, AppError> {
        self.provider_repo
            .find_within_radius(target_location, radius_meters, service_type)
            .await
    }
}
