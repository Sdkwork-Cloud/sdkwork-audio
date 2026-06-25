//! Speech service models

use serde::{Deserialize, Serialize};

/// Speech synthesis request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechSynthesisRequest {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub text: String,
    pub text_format: TextFormat,
    pub language: Option<String>,
    pub voice_id: Option<String>,
    pub speed: f64,
    pub pitch: f64,
    pub volume: f64,
    pub emotion: Option<String>,
    pub emotion_intensity: f64,
    pub audio_format: AudioFormat,
    pub sample_rate: u32,
    pub idempotency_key: Option<String>,
}

/// Text format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TextFormat {
    Plain,
    SSML,
}

impl TextFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextFormat::Plain => "plain",
            TextFormat::SSML => "ssml",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "plain" => Some(TextFormat::Plain),
            "ssml" => Some(TextFormat::SSML),
            _ => None,
        }
    }
}

/// Audio format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AudioFormat {
    MP3,
    WAV,
    FLAC,
    OGG,
}

impl AudioFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            AudioFormat::MP3 => "mp3",
            AudioFormat::WAV => "wav",
            AudioFormat::FLAC => "flac",
            AudioFormat::OGG => "ogg",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "mp3" => Some(AudioFormat::MP3),
            "wav" => Some(AudioFormat::WAV),
            "flac" => Some(AudioFormat::FLAC),
            "ogg" => Some(AudioFormat::OGG),
            _ => None,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            AudioFormat::MP3 => "audio/mpeg",
            AudioFormat::WAV => "audio/wav",
            AudioFormat::FLAC => "audio/flac",
            AudioFormat::OGG => "audio/ogg",
        }
    }
}

/// Speech synthesis response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechSynthesisResponse {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub estimated_duration_ms: Option<u64>,
}

/// Speech synthesis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechSynthesisResult {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub audio_url: Option<String>,
    pub duration_ms: Option<u64>,
    pub file_size_bytes: Option<u64>,
    pub mime_type: Option<String>,
}

/// Voice info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInfo {
    pub voice_id: String,
    pub voice_no: String,
    pub name: String,
    pub description: Option<String>,
    pub language: String,
    pub gender: Option<String>,
    pub voice_type: String,
    pub preview_url: Option<String>,
}

/// Voice list request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceListRequest {
    pub tenant_id: i64,
    pub user_id: Option<i64>,
    pub language: Option<String>,
    pub gender: Option<String>,
    pub voice_type: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

/// Voice list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceListResponse {
    pub voices: Vec<VoiceInfo>,
    pub total: i64,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}
