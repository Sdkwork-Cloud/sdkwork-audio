//! Sound effect service models

use serde::{Deserialize, Serialize};

/// Sound effect request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectRequest {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub description: String,
    pub duration_ms: Option<u64>,
    pub style: Option<String>,
    pub intensity: f64,
    pub audio_format: AudioFormat,
    pub sample_rate: u32,
    pub idempotency_key: Option<String>,
}

/// Audio format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AudioFormat {
    WAV,
    MP3,
    FLAC,
    OGG,
}

impl AudioFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            AudioFormat::WAV => "wav",
            AudioFormat::MP3 => "mp3",
            AudioFormat::FLAC => "flac",
            AudioFormat::OGG => "ogg",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "wav" => Some(AudioFormat::WAV),
            "mp3" => Some(AudioFormat::MP3),
            "flac" => Some(AudioFormat::FLAC),
            "ogg" => Some(AudioFormat::OGG),
            _ => None,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            AudioFormat::WAV => "audio/wav",
            AudioFormat::MP3 => "audio/mpeg",
            AudioFormat::FLAC => "audio/flac",
            AudioFormat::OGG => "audio/ogg",
        }
    }
}

/// Sound effect response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectResponse {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub estimated_duration_ms: Option<u64>,
}

/// Sound effect result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectResult {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub audio_url: Option<String>,
    pub duration_ms: Option<u64>,
    pub file_size_bytes: Option<u64>,
    pub mime_type: Option<String>,
    pub format: Option<String>,
    pub sample_rate: Option<u32>,
}

/// Sound effect task detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectTaskDetail {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub progress: i32,
    pub request_params: SoundEffectRequest,
    pub result: Option<SoundEffectResult>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// Sound effect list request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectListRequest {
    pub tenant_id: i64,
    pub user_id: Option<i64>,
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

/// Sound effect list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectListResponse {
    pub tasks: Vec<SoundEffectTaskDetail>,
    pub total: i64,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

/// Sound effect preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectPreset {
    pub preset_id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub default_duration_ms: u64,
    pub default_style: Option<String>,
}

/// Sound effect presets response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectPresetsResponse {
    pub presets: Vec<SoundEffectPreset>,
}

/// Sound effect category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectCategory {
    pub category_id: String,
    pub name: String,
    pub description: String,
    pub preset_count: u32,
}

/// Sound effect categories response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEffectCategoriesResponse {
    pub categories: Vec<SoundEffectCategory>,
}
