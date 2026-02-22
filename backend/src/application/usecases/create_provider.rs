use crate::application::provider_repo::ProviderRepository;
use crate::domain::error::AppError;
use crate::domain::provider_profile::ProviderProfile;
use log::info;

pub struct CreateProviderUseCase<'a, R: ProviderRepository> {
    pub provider_repo: &'a R,
}

impl<'a, R: ProviderRepository> CreateProviderUseCase<'a, R> {
    pub async fn execute(&self, profile: &ProviderProfile) -> Result<(), AppError> {
        info!("Creating new provider profile for user {}", profile.user_id);

        // 1. Validate data here if necessary (domain rules)

        // 2. Persist to DB
        self.provider_repo.create_provider(profile).await?;

        Ok(())
    }
}
