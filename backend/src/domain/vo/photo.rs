use crate::domain::vo::url::Url;

#[derive(Debug, Clone)]
pub struct Photo {
    pub url: Url,
    pub title: Option<String>,
}
