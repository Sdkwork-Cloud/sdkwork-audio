//! SDKWork Audio real-time processing
//!
//! This crate provides real-time audio processing capabilities.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Real-time session type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RealtimeSessionType {
    Transcription,
    Translation,
    Call,
}

impl RealtimeSessionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RealtimeSessionType::Transcription => "transcription",
            RealtimeSessionType::Translation => "translation",
            RealtimeSessionType::Call => "call",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "transcription" => Some(RealtimeSessionType::Transcription),
            "translation" => Some(RealtimeSessionType::Translation),
            "call" => Some(RealtimeSessionType::Call),
            _ => None,
        }
    }
}

/// Real-time session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RealtimeSessionStatus {
    Created,
    Connecting,
    Active,
    Paused,
    Ended,
    Failed,
}

impl RealtimeSessionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RealtimeSessionStatus::Created => "created",
            RealtimeSessionStatus::Connecting => "connecting",
            RealtimeSessionStatus::Active => "active",
            RealtimeSessionStatus::Paused => "paused",
            RealtimeSessionStatus::Ended => "ended",
            RealtimeSessionStatus::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "created" => Some(RealtimeSessionStatus::Created),
            "connecting" => Some(RealtimeSessionStatus::Connecting),
            "active" => Some(RealtimeSessionStatus::Active),
            "paused" => Some(RealtimeSessionStatus::Paused),
            "ended" => Some(RealtimeSessionStatus::Ended),
            "failed" => Some(RealtimeSessionStatus::Failed),
            _ => None,
        }
    }
}

/// Real-time session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeSessionConfig {
    pub session_type: RealtimeSessionType,
    pub language: Option<String>,
    pub target_language: Option<String>,
    pub enable_translation: bool,
    pub enable_speaker_diarization: bool,
    pub max_speakers: Option<u32>,
    pub is_recording: bool,
}

/// Real-time session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeSession {
    pub session_id: String,
    pub session_type: RealtimeSessionType,
    pub status: RealtimeSessionStatus,
    pub config: RealtimeSessionConfig,
    pub websocket_url: Option<String>,
    pub participant_count: u32,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Real-time transcription result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranscriptionResult {
    pub segment_id: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
    pub speaker_id: Option<u32>,
    pub speaker_label: Option<String>,
    pub confidence: f64,
    pub words: Vec<RealtimeWord>,
}

/// Real-time word
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeWord {
    pub word: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub confidence: f64,
}

/// Real-time translation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeTranslationResult {
    pub segment_id: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub source_text: String,
    pub translated_text: String,
    pub source_language: String,
    pub target_language: String,
    pub confidence: f64,
}

/// Real-time event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealtimeEvent {
    SessionCreated {
        session_id: String,
        websocket_url: String,
    },
    SessionStarted {
        session_id: String,
    },
    TranscriptionResult {
        session_id: String,
        result: RealtimeTranscriptionResult,
    },
    TranslationResult {
        session_id: String,
        result: RealtimeTranslationResult,
    },
    SessionEnded {
        session_id: String,
    },
    Error {
        session_id: String,
        error_code: String,
        error_message: String,
    },
}

/// Real-time service trait
#[async_trait]
pub trait RealtimeAudioService {
    /// Create a new real-time session
    async fn create_session(
        &self,
        config: RealtimeSessionConfig,
    ) -> Result<RealtimeSession, Box<dyn std::error::Error + Send + Sync>>;

    /// Get session by ID
    async fn get_session(
        &self,
        session_id: &str,
    ) -> Result<Option<RealtimeSession>, Box<dyn std::error::Error + Send + Sync>>;

    /// End a session
    async fn end_session(
        &self,
        session_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Mock real-time service for testing
pub struct MockRealtimeAudioService;

#[async_trait]
impl RealtimeAudioService for MockRealtimeAudioService {
    async fn create_session(
        &self,
        config: RealtimeSessionConfig,
    ) -> Result<RealtimeSession, Box<dyn std::error::Error + Send + Sync>> {
        Ok(RealtimeSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            session_type: config.session_type.clone(),
            status: RealtimeSessionStatus::Created,
            config,
            websocket_url: Some("wss://example.com/ws".to_string()),
            participant_count: 0,
            started_at: None,
            ended_at: None,
        })
    }

    async fn get_session(
        &self,
        _session_id: &str,
    ) -> Result<Option<RealtimeSession>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    async fn end_session(
        &self,
        _session_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}
