use chrono::DateTime as ChonoDateTime;

#[derive(Debug, Clone)]
pub struct DateTime(ChonoDateTime<chrono::Utc>);

impl Default for DateTime {
    fn default() -> Self {
        Self::new()
    }
}

impl DateTime {
    pub fn new() -> Self {
        Self(chrono::Utc::now())
    }

    pub fn new_from_date_time(dt: ChonoDateTime<chrono::Utc>) -> Self {
        Self(dt)
    }

    pub fn value(&self) -> String {
        self.0.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn to_chono_date_time(&self) -> ChonoDateTime<chrono::Utc> {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Date(chrono::NaiveDate);

impl Date {
    pub fn new() -> Self {
        Self(chrono::Utc::now().naive_utc().date())
    }

    pub fn value(&self) -> String {
        self.0.format("%Y-%m-%d").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datetime_new_works_and_formats() {
        let dt = DateTime::new();
        let s = dt.value();
        // Expect a non-empty formatted string like "YYYY-MM-DD HH:MM:SS"
        assert!(!s.is_empty(), "formatted DateTime should not be empty");
        assert!(
            s.len() >= 19,
            "formatted DateTime should have at least 19 chars, got {}",
            s.len()
        );
    }

    #[test]
    fn date_new_works_and_formats() {
        let d = Date::new();
        let s = d.value();
        // Expect a non-empty formatted string like "YYYY-MM-DD"
        assert!(!s.is_empty(), "formatted Date should not be empty");
        assert!(
            s.len() >= 10,
            "formatted Date should have at least 10 chars, got {}",
            s.len()
        );
    }
}
