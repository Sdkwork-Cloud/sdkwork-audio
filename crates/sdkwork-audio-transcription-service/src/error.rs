//! Transcription service errors

use thiserror::Error;

/// Transcription service error
#[derive(Error, Debug)]
pub enum TranscriptionServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("AI engine error: {0}")]
    AiEngine(String),

    #[error("Drive service error: {0}")]
    DriveService(String),

    #[error("Task not found: {task_no}")]
    TaskNotFound { task_no: String },

    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },

    #[error("Provider error: {provider_code}: {message}")]
    ProviderError {
        provider_code: String,
        message: String,
    },

    #[error("Task already exists: {idempotency_key}")]
    TaskAlreadyExists { idempotency_key: String },

    #[error("Audio download error: {0}")]
    AudioDownloadError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for transcription service
pub type TranscriptionServiceResult<T> = Result<T, TranscriptionServiceError>;
