use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub business_name: String,
    pub description: Option<String>,
    pub services_provided: Vec<String>,
    pub location: Location,
    pub tags: Vec<String>,
    pub photos: Vec<String>,
    pub videos: Vec<String>,
    pub operating_hours: Option<serde_json::Value>,
    pub ranking_score: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
