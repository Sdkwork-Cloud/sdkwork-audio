//! SDKWork Audio artifact drive service
//!
//! This crate provides drive integration for audio artifacts.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Drive sync status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DriveSyncStatus {
    PendingUpload,
    Uploading,
    Uploaded,
    Failed,
    Skipped,
    Deleted,
}

impl DriveSyncStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DriveSyncStatus::PendingUpload => "pending_upload",
            DriveSyncStatus::Uploading => "uploading",
            DriveSyncStatus::Uploaded => "uploaded",
            DriveSyncStatus::Failed => "failed",
            DriveSyncStatus::Skipped => "skipped",
            DriveSyncStatus::Deleted => "deleted",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending_upload" => Some(DriveSyncStatus::PendingUpload),
            "uploading" => Some(DriveSyncStatus::Uploading),
            "uploaded" => Some(DriveSyncStatus::Uploaded),
            "failed" => Some(DriveSyncStatus::Failed),
            "skipped" => Some(DriveSyncStatus::Skipped),
            "deleted" => Some(DriveSyncStatus::Deleted),
            _ => None,
        }
    }
}

/// Drive upload result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveUploadResult {
    pub drive_space_id: String,
    pub drive_node_id: String,
    pub drive_resource_uri: String,
    pub upload_item_id: Option<String>,
    pub upload_session_id: Option<String>,
}

/// Drive service trait
#[async_trait]
pub trait AudioArtifactDriveService {
    /// Upload artifact to drive
    async fn upload_artifact(
        &self,
        tenant_id: i64,
        user_id: i64,
        artifact_data: &[u8],
        mime_type: &str,
        filename: &str,
    ) -> Result<DriveUploadResult, Box<dyn std::error::Error + Send + Sync>>;

    /// Get download URL for artifact
    async fn get_download_url(
        &self,
        drive_space_id: &str,
        drive_node_id: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    /// Delete artifact from drive
    async fn delete_artifact(
        &self,
        drive_space_id: &str,
        drive_node_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Mock drive service for testing
pub struct MockAudioArtifactDriveService;

#[async_trait]
impl AudioArtifactDriveService for MockAudioArtifactDriveService {
    async fn upload_artifact(
        &self,
        _tenant_id: i64,
        _user_id: i64,
        _artifact_data: &[u8],
        _mime_type: &str,
        _filename: &str,
    ) -> Result<DriveUploadResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(DriveUploadResult {
            drive_space_id: "mock-space".to_string(),
            drive_node_id: "mock-node".to_string(),
            drive_resource_uri: "drive://spaces/mock-space/nodes/mock-node".to_string(),
            upload_item_id: Some("mock-item".to_string()),
            upload_session_id: Some("mock-session".to_string()),
        })
    }

    async fn get_download_url(
        &self,
        _drive_space_id: &str,
        _drive_node_id: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("https://example.com/download".to_string())
    }

    async fn delete_artifact(
        &self,
        _drive_space_id: &str,
        _drive_node_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}
