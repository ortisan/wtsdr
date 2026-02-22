use crate::application::provider_repo::ProviderRepository;
use crate::domain::{error::AppError, provider_profile::ProviderProfile, request::ServiceRequest};
use log::info;

pub struct BroadcastServiceRequestUseCase<'a, R: ProviderRepository> {
    pub provider_repo: &'a R,
}

impl<'a, R: ProviderRepository> BroadcastServiceRequestUseCase<'a, R> {
    pub async fn execute(
        &self,
        request: &ServiceRequest,
    ) -> Result<Vec<ProviderProfile>, AppError> {
        info!(
            "Executing BroadcastServiceRequestUseCase for request ID: {}",
            request.id
        );

        // 1. Find all providers matching the service type within the requested radius
        let matched_providers = self
            .provider_repo
            .find_within_radius(
                &request.target_location,
                request.radius_meters,
                &request.service_type,
            )
            .await?;

        info!(
            "Found {} providers within {} meters offering {:?}",
            matched_providers.len(),
            request.radius_meters,
            request.service_type
        );

        // 2. Here we would enqueue the event to a message broker (e.g., RabbitMQ, Redis)
        // For now, we mock the asynchronous messaging queue by logging
        for provider in &matched_providers {
            info!(
                "Enqueuing push notification to Provider: {} ({})",
                provider.id, provider.business_name
            );
        }

        // Return the matched providers to the controller
        Ok(matched_providers)
    }
}
