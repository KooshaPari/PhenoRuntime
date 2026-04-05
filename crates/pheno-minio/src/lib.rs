//! MinIO client for object storage (S3-compatible)
//!
//! MinIO is a high-performance, S3-compatible object storage.

use aws_sdk_s3::{Client, primitives::ByteStream};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, info};

/// MinIO errors
#[derive(Error, Debug)]
pub enum MinioError {
    #[error("connection error: {0}")]
    Connection(String),

    #[error("upload error: {0}")]
    Upload(String),

    #[error("download error: {0}")]
    Download(String),

    #[error("delete error: {0}")]
    Delete(String),

    #[error("not found: {0}")]
    NotFound(String),
}

/// Object metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: u64,
    pub etag: String,
    pub last_modified: Option<DateTime<Utc>>,
}

/// MinIO/S3 client
#[derive(Clone)]
pub struct MinioClient {
    client: Client,
    bucket: String,
}

impl MinioClient {
    /// Create new MinIO client
    pub async fn new(endpoint: &str, bucket: &str) -> Result<Self, MinioError> {
        info!("Connecting to MinIO: {} (bucket: {})", endpoint, bucket);

        // Use default credentials (for MinIO local)
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .credentials_provider(aws_config::Credentials::new(
                "minioadmin",
                "minioadmin123",
                None,
                None,
                "static",
            ))
            .endpoint_url(endpoint)
            .force_path_style(true)
            .load()
            .await;

        let client = Client::new(&config);

        // Ensure bucket exists (ignore error if already exists)
        let _ = client.create_bucket().bucket(bucket).send().await;

        info!("MinIO connection established, bucket '{}' ready", bucket);
        Ok(Self {
            client,
            bucket: bucket.to_string(),
        })
    }

    /// Upload object
    pub async fn put_object(&self, key: &str, data: Bytes) -> Result<(), MinioError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(data))
            .send()
            .await
            .map_err(|e| MinioError::Upload(e.to_string()))?;

        debug!("Object uploaded: {}/{}", self.bucket, key);
        Ok(())
    }

    /// Download object
    pub async fn get_object(&self, key: &str) -> Result<Bytes, MinioError> {
        match self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(response) => {
                let bytes = response
                    .body
                    .collect()
                    .await
                    .map_err(|e| MinioError::Download(e.to_string()))?;
                debug!(
                    "Object downloaded: {}/{} ({} bytes)",
                    self.bucket,
                    key,
                    bytes.len()
                );
                Ok(bytes.into_bytes())
            }
            Err(e) => {
                if e.to_string().contains("NoSuchKey") {
                    Err(MinioError::NotFound(key.to_string()))
                } else {
                    Err(MinioError::Download(e.to_string()))
                }
            }
        }
    }

    /// Delete object
    pub async fn delete_object(&self, key: &str) -> Result<(), MinioError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| MinioError::Delete(e.to_string()))?;

        debug!("Object deleted: {}/{}", self.bucket, key);
        Ok(())
    }

    /// List objects
    pub async fn list_objects(&self, prefix: Option<&str>) -> Result<Vec<ObjectMetadata>, MinioError> {
        let mut request = self.client.list_objects_v2().bucket(&self.bucket);

        if let Some(p) = prefix {
            request = request.prefix(p);
        }

        let response = request.send().await.map_err(|e| MinioError::Download(e.to_string()))?;

        let objects = response
            .contents()
            .unwrap_or_default()
            .iter()
            .filter_map(|obj| {
                Some(ObjectMetadata {
                    key: obj.key()?.to_string(),
                    size: obj.size() as u64,
                    etag: obj.e_tag()?.trim_matches('"').to_string(),
                    last_modified: obj.last_modified().cloned(),
                })
            })
            .collect();

        Ok(objects)
    }

    /// Check if object exists
    pub async fn exists(&self, key: &str) -> Result<bool, MinioError> {
        match self.get_object(key).await {
            Ok(_) => Ok(true),
            Err(MinioError::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Generate presigned URL (expires in given duration)
    pub async fn presigned_url(&self, key: &str, expires: Duration) -> Result<String, MinioError> {
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(expires)
            .build()
            .map_err(|e| MinioError::Connection(e.to_string()))?;

        let presigned = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| MinioError::Connection(e.to_string()))?;

        Ok(presigned.uri().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_metadata() {
        let meta = ObjectMetadata {
            key: "test.txt".to_string(),
            size: 1024,
            etag: "abc123".to_string(),
            last_modified: Some(Utc::now()),
        };

        let json = serde_json::to_string(&meta).unwrap();
        let parsed: ObjectMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(meta.key, parsed.key);
        assert_eq!(meta.size, parsed.size);
    }
}
