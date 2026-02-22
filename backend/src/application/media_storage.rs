use crate::domain::error::AppError;
use async_trait::async_trait;
use bytes::Bytes;

pub struct UploadResult {
    pub url: String,
}

#[async_trait]
pub trait MediaStorage: Send + Sync {
    async fn upload_file(
        &self,
        key: &str,
        content: Bytes,
        content_type: &str,
    ) -> Result<UploadResult, AppError>;

    async fn delete_file(&self, key: &str) -> Result<(), AppError>;
}
