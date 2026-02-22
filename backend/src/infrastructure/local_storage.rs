use crate::application::media_storage::{MediaStorage, UploadResult};
use crate::domain::error::AppError;
use async_trait::async_trait;
use bytes::Bytes;
use std::fs;
use std::path::PathBuf;

pub struct LocalDiskStorageAdapter {
    base_folder: PathBuf,
    base_url: String,
}

impl LocalDiskStorageAdapter {
    pub fn new(base_folder: String, base_url: String) -> Self {
        let path = PathBuf::from(&base_folder);
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create local storage directory");
        }
        Self {
            base_folder: path,
            base_url,
        }
    }
}

#[async_trait]
impl MediaStorage for LocalDiskStorageAdapter {
    async fn upload_file(
        &self,
        key: &str,
        content: Bytes,
        _content_type: &str,
    ) -> Result<UploadResult, AppError> {
        let file_path = self.base_folder.join(key);

        let mut dest_dir = file_path.clone();
        dest_dir.pop();
        if !dest_dir.exists() {
            tokio::fs::create_dir_all(&dest_dir).await.map_err(|e| {
                AppError::InternalServerError(format!("Local storage error: {}", e))
            })?;
        }

        tokio::fs::write(&file_path, content)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Local save failed: {}", e)))?;

        let url = format!("{}/{}", self.base_url, key);

        Ok(UploadResult { url })
    }

    async fn delete_file(&self, key: &str) -> Result<(), AppError> {
        let file_path = self.base_folder.join(key);
        if file_path.exists() {
            tokio::fs::remove_file(file_path).await.map_err(|e| {
                AppError::InternalServerError(format!("Local delete failed: {}", e))
            })?;
        }
        Ok(())
    }
}
