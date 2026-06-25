//! Database models for audio generation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Audio generation task
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioGenerationTask {
    pub id: i64,
    pub task_no: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub operation_type: String,
    pub provider_code: String,
    pub provider_route_id: Option<i64>,
    pub model: Option<String>,
    pub provider_task_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub input_hash: Option<String>,
    pub status: String,
    pub progress: i32,
    pub request_json: String,
    pub normalized_options_json: Option<String>,
    pub provider_request_json: Option<String>,
    pub provider_response_json: Option<String>,
    pub result_json: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub callback_url: Option<String>,
    pub callback_status: Option<String>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio task event
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioTaskEvent {
    pub id: i64,
    pub event_no: String,
    pub task_id: i64,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: Option<String>,
    pub provider_code: Option<String>,
    pub provider_event_id: Option<String>,
    pub provider_task_id: Option<String>,
    pub payload_hash: Option<String>,
    pub payload_json: String,
    pub received_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub status: String,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio artifact
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioArtifact {
    pub id: i64,
    pub artifact_no: String,
    pub task_id: Option<i64>,
    pub request_id: Option<String>,
    pub kind: String,
    pub artifact_type: Option<String>,
    pub title: Option<String>,
    pub voice_id: Option<String>,
    pub provider_code: Option<String>,
    pub provider_asset_id: Option<String>,
    pub artifact_index: i32,
    pub format: Option<String>,
    pub mime_type: Option<String>,
    pub duration_seconds: Option<i32>,
    pub checksum_json: Option<String>,
    pub transcript_text: Option<String>,
    pub translation_text: Option<String>,
    pub media_resource_json: String,
    pub resource_snapshot_json: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio artifact drive sync
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioArtifactDriveSync {
    pub id: i64,
    pub sync_no: String,
    pub task_id: i64,
    pub artifact_id: i64,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub anonymous_id: Option<String>,
    pub actor_type: String,
    pub provider_code: Option<String>,
    pub provider_asset_id: Option<String>,
    pub artifact_index: i32,
    pub source_uri: Option<String>,
    pub source_hash: Option<String>,
    pub drive_space_type: String,
    pub drive_space_id: Option<String>,
    pub drive_node_id: Option<String>,
    pub drive_upload_item_id: Option<String>,
    pub drive_upload_session_id: Option<String>,
    pub drive_resource_json: Option<String>,
    pub sync_status: String,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub uploaded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio provider route
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioProviderRoute {
    pub id: i64,
    pub route_key: String,
    pub route_name: String,
    pub provider_id: String,
    pub provider_type: String,
    pub client_protocol: String,
    pub upstream_protocol: String,
    pub upstream_config_json: String,
    pub capabilities_json: String,
    pub enabled: bool,
    pub health_status: String,
    pub managed_by: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio provider route capability
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioProviderRouteCapability {
    pub id: i64,
    pub route_id: i64,
    pub capability: String,
    pub operation_set_json: String,
    pub streaming: bool,
    pub timeout_ms: Option<i64>,
    pub request_policy_ref: Option<String>,
    pub response_policy_ref: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio provider webhook event
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioProviderWebhookEvent {
    pub id: i64,
    pub event_no: String,
    pub provider_code: String,
    pub event_id: String,
    pub task_id: Option<i64>,
    pub provider_task_id: Option<String>,
    pub signature_status: String,
    pub payload_hash: String,
    pub payload_json: String,
    pub processing_status: String,
    pub attempt_count: i32,
    pub received_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub error_summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio webhook delivery
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioWebhookDelivery {
    pub id: i64,
    pub delivery_no: String,
    pub task_id: i64,
    pub event_type: String,
    pub target_url: String,
    pub delivery_status: String,
    pub attempt_count: i32,
    pub last_status_code: Option<i32>,
    pub last_error: Option<String>,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub payload_json: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio request log
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioRequestLog {
    pub id: i64,
    pub request_no: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub task_id: Option<i64>,
    pub provider_code: Option<String>,
    pub request_method: String,
    pub request_url: String,
    pub request_headers_json: Option<String>,
    pub request_body_json: Option<String>,
    pub response_status: Option<i32>,
    pub response_headers_json: Option<String>,
    pub response_body_json: Option<String>,
    pub duration_ms: Option<i64>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio voice
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioVoice {
    pub id: i64,
    pub voice_no: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub language: String,
    pub gender: Option<String>,
    pub voice_type: String,
    pub provider_code: Option<String>,
    pub provider_voice_id: Option<String>,
    pub provider_config_json: Option<String>,
    pub clone_reference_url: Option<String>,
    pub clone_parameters_json: Option<String>,
    pub status: String,
    pub is_public: bool,
    pub usage_count: i64,
    pub quality_score: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio realtime session
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioRealtimeSession {
    pub id: i64,
    pub session_no: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub session_type: String,
    pub title: Option<String>,
    pub language: Option<String>,
    pub target_language: Option<String>,
    pub enable_translation: bool,
    pub enable_speaker_diarization: bool,
    pub connection_id: Option<String>,
    pub websocket_url: Option<String>,
    pub status: String,
    pub participant_count: i32,
    pub max_participants: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub is_recording: bool,
    pub recording_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio workspace
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioWorkspace {
    pub id: i64,
    pub workspace_no: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub settings_json: Option<String>,
    pub is_public: bool,
    pub collaborator_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio workspace track
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioWorkspaceTrack {
    pub id: i64,
    pub track_no: String,
    pub workspace_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub track_index: i32,
    pub settings_json: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Audio workspace clip
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioWorkspaceClip {
    pub id: i64,
    pub clip_no: String,
    pub track_id: i64,
    pub artifact_id: Option<i64>,
    pub name: Option<String>,
    pub start_ms: i64,
    pub end_ms: i64,
    pub waveform_data_json: Option<String>,
    pub settings_json: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Task status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Queued,
    Routing,
    Submitted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    Expired,
    NeedsReview,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Queued => "queued",
            TaskStatus::Routing => "routing",
            TaskStatus::Submitted => "submitted",
            TaskStatus::Running => "running",
            TaskStatus::Succeeded => "succeeded",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
            TaskStatus::Expired => "expired",
            TaskStatus::NeedsReview => "needs_review",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "queued" => Some(TaskStatus::Queued),
            "routing" => Some(TaskStatus::Routing),
            "submitted" => Some(TaskStatus::Submitted),
            "running" => Some(TaskStatus::Running),
            "succeeded" => Some(TaskStatus::Succeeded),
            "failed" => Some(TaskStatus::Failed),
            "cancelled" => Some(TaskStatus::Cancelled),
            "expired" => Some(TaskStatus::Expired),
            "needs_review" => Some(TaskStatus::NeedsReview),
            _ => None,
        }
    }
}

/// Operation type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationType {
    Speech,
    Transcription,
    Translation,
    SoundEffect,
    Music,
}

impl OperationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OperationType::Speech => "speech",
            OperationType::Transcription => "transcription",
            OperationType::Translation => "translation",
            OperationType::SoundEffect => "sound_effect",
            OperationType::Music => "music",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "speech" => Some(OperationType::Speech),
            "transcription" => Some(OperationType::Transcription),
            "translation" => Some(OperationType::Translation),
            "sound_effect" => Some(OperationType::SoundEffect),
            "music" => Some(OperationType::Music),
            _ => None,
        }
    }
}
