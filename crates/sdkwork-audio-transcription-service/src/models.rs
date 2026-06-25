//! Transcription service models

use serde::{Deserialize, Serialize};

/// Transcription request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionRequest {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub audio_url: String,
    pub language: Option<String>,
    pub detect_language: bool,
    pub enable_timestamps: bool,
    pub enable_speaker_diarization: bool,
    pub max_speakers: Option<u32>,
    pub output_format: OutputFormat,
    pub include_confidence: bool,
    pub include_words: bool,
    pub idempotency_key: Option<String>,
}

/// Output format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    JSON,
    SRT,
    VTT,
    TXT,
}

impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::JSON => "json",
            OutputFormat::SRT => "srt",
            OutputFormat::VTT => "vtt",
            OutputFormat::TXT => "txt",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "json" => Some(OutputFormat::JSON),
            "srt" => Some(OutputFormat::SRT),
            "vtt" => Some(OutputFormat::VTT),
            "txt" => Some(OutputFormat::TXT),
            _ => None,
        }
    }
}

/// Transcription response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResponse {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub estimated_duration_ms: Option<u64>,
}

/// Transcription result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub text: Option<String>,
    pub language: Option<String>,
    pub segments: Option<Vec<TranscriptionSegment>>,
    pub confidence: Option<f64>,
    pub duration_ms: Option<u64>,
    pub output_url: Option<String>,
}

/// Transcription segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub segment_id: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
    pub speaker_id: Option<u32>,
    pub speaker_label: Option<String>,
    pub confidence: f64,
    pub words: Option<Vec<TranscriptionWord>>,
}

/// Transcription word
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionWord {
    pub word: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub confidence: f64,
}

/// Transcription task detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionTaskDetail {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub progress: i32,
    pub request_params: TranscriptionRequest,
    pub result: Option<TranscriptionResult>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// Transcription list request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionListRequest {
    pub tenant_id: i64,
    pub user_id: Option<i64>,
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

/// Transcription list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionListResponse {
    pub tasks: Vec<TranscriptionTaskDetail>,
    pub total: i64,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}
