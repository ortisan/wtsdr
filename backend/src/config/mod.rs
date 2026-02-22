use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum StorageMode {
    Local,
    S3,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub server_bind: String,
    pub storage_mode: StorageMode,
    pub local_storage_path: String,
    pub local_storage_url: String,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let server_bind = env::var("SERVER_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

        let storage_mode = match env::var("STORAGE_MODE")
            .unwrap_or_else(|_| "local".to_string())
            .as_str()
        {
            "s3" => StorageMode::S3,
            _ => StorageMode::Local,
        };

        let local_storage_path =
            env::var("LOCAL_STORAGE_PATH").unwrap_or_else(|_| "./uploads".to_string());
        let local_storage_url = env::var("LOCAL_STORAGE_URL")
            .unwrap_or_else(|_| "http://localhost:8080/uploads".to_string());
        let s3_bucket = env::var("S3_BUCKET").ok();
        let s3_region = env::var("S3_REGION").ok();

        Self {
            database_url,
            server_bind,
            storage_mode,
            local_storage_path,
            local_storage_url,
            s3_bucket,
            s3_region,
        }
    }
}
