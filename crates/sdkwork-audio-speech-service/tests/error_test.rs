//! Unit tests for speech service errors

use sdkwork_audio_speech_service::error::*;

#[test]
fn test_error_display() {
    let error = SpeechServiceError::TaskNotFound {
        task_no: "uuid-123".to_string(),
    };
    assert_eq!(error.to_string(), "Task not found: uuid-123");

    let error = SpeechServiceError::VoiceNotFound {
        voice_id: "voice-456".to_string(),
    };
    assert_eq!(error.to_string(), "Voice not found: voice-456");

    let error = SpeechServiceError::InvalidRequest {
        message: "Text is required".to_string(),
    };
    assert_eq!(error.to_string(), "Invalid request: Text is required");

    let error = SpeechServiceError::ProviderError {
        provider_code: "openai".to_string(),
        message: "API rate limit exceeded".to_string(),
    };
    assert_eq!(
        error.to_string(),
        "Provider error: openai: API rate limit exceeded"
    );

    let error = SpeechServiceError::TaskAlreadyExists {
        idempotency_key: "key-123".to_string(),
    };
    assert_eq!(
        error.to_string(),
        "Task already exists: key-123"
    );

    let error = SpeechServiceError::AiEngine("AI engine error".to_string());
    assert_eq!(error.to_string(), "AI engine error: AI engine error");

    let error = SpeechServiceError::DriveService("Drive error".to_string());
    assert_eq!(error.to_string(), "Drive service error: Drive error");

    let error = SpeechServiceError::Internal("Internal error".to_string());
    assert_eq!(error.to_string(), "Internal error: Internal error");
}

#[test]
fn test_error_debug() {
    let error = SpeechServiceError::TaskNotFound {
        task_no: "uuid-123".to_string(),
    };
    let debug = format!("{:?}", error);
    assert!(debug.contains("TaskNotFound"));
    assert!(debug.contains("uuid-123"));
}

#[test]
fn test_error_from_sqlx() {
    let sqlx_error = sqlx::Error::RowNotFound;
    let error: SpeechServiceError = sqlx_error.into();
    match error {
        SpeechServiceError::Database(_) => {}
        _ => panic!("Expected Database error"),
    }
}

#[test]
fn test_result_type() {
    let success: SpeechServiceResult<i32> = Ok(42);
    assert!(success.is_ok());
    assert_eq!(success.unwrap(), 42);

    let failure: SpeechServiceResult<i32> = Err(SpeechServiceError::Internal("error".to_string()));
    assert!(failure.is_err());
}
