use crate::domain::provider_profile::Location;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub description: String,
    pub target_location: Location,
    pub service_type: String,
    pub radius_meters: f64,
    pub created_at: DateTime<Utc>,
}
