//! Translation service models

use serde::{Deserialize, Serialize};

/// Translation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub audio_url: String,
    pub source_language: Option<String>,
    pub target_language: String,
    pub output_format: OutputFormat,
    pub include_timestamps: bool,
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

/// Translation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub estimated_duration_ms: Option<u64>,
}

/// Translation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub source_text: Option<String>,
    pub translated_text: Option<String>,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub confidence: Option<f64>,
    pub segments: Option<Vec<TranslationSegment>>,
    pub output_url: Option<String>,
}

/// Translation segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationSegment {
    pub segment_id: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub source_text: String,
    pub translated_text: String,
    pub confidence: f64,
}

/// Translation task detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationTaskDetail {
    pub task_id: String,
    pub task_no: String,
    pub status: String,
    pub progress: i32,
    pub request_params: TranslationRequest,
    pub result: Option<TranslationResult>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// Translation list request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationListRequest {
    pub tenant_id: i64,
    pub user_id: Option<i64>,
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

/// Translation list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationListResponse {
    pub tasks: Vec<TranslationTaskDetail>,
    pub total: i64,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

/// Supported language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedLanguage {
    pub code: String,
    pub name: String,
    pub native_name: String,
}

/// Supported languages response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedLanguagesResponse {
    pub languages: Vec<SupportedLanguage>,
}
