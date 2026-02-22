use crate::application::media_storage::{MediaStorage, UploadResult};
use crate::domain::error::AppError;
use async_trait::async_trait;
use aws_sdk_s3::Client;
use bytes::Bytes;

pub struct S3StorageAdapter {
    client: Client,
    bucket_name: String,
    region: String,
}

impl S3StorageAdapter {
    pub async fn new(bucket_name: String, region: String) -> Self {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        Self {
            client,
            bucket_name,
            region,
        }
    }
}

#[async_trait]
impl MediaStorage for S3StorageAdapter {
    async fn upload_file(
        &self,
        key: &str,
        content: Bytes,
        content_type: &str,
    ) -> Result<UploadResult, AppError> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(content.into())
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("S3 upload failed: {}", e)))?;

        // Construct the public URL assuming the bucket is public-read
        // Or we could return a signed URL depending on business logics.
        let url = format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.bucket_name, self.region, key
        );

        Ok(UploadResult { url })
    }

    async fn delete_file(&self, key: &str) -> Result<(), AppError> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("S3 deletion failed: {}", e)))?;

        Ok(())
    }
}
