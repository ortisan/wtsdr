use crate::common::error::{AppError, ErrorData};
use crate::common::result::ResultApp;
use std::sync::Arc;

type Degrees = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint {
    pub lat: Degrees, // degrees
    pub lon: Degrees, // degrees
}

impl GeoPoint {
    pub fn new(lat: Degrees, lon: Degrees) -> ResultApp<Self> {
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            return Err(Arc::new(AppError::Validation(ErrorData::new(
                "invalid-latitude-longitude",
                "Invalid latitude/longitude",
            ))));
        }
        Ok(GeoPoint { lat, lon })
    }

    pub fn value(&self) -> (Degrees, Degrees) {
        (self.lat, self.lon)
    }
}
