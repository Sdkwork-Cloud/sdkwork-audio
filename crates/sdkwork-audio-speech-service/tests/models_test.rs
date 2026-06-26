//! Unit tests for speech service models

use sdkwork_audio_speech_service::models::*;

#[test]
fn test_text_format_as_str() {
    assert_eq!(TextFormat::Plain.as_str(), "plain");
    assert_eq!(TextFormat::SSML.as_str(), "ssml");
}

#[test]
fn test_text_format_from_str() {
    assert_eq!(TextFormat::from_str("plain"), Some(TextFormat::Plain));
    assert_eq!(TextFormat::from_str("ssml"), Some(TextFormat::SSML));
    assert_eq!(TextFormat::from_str("invalid"), None);
}

#[test]
fn test_audio_format_as_str() {
    assert_eq!(AudioFormat::MP3.as_str(), "mp3");
    assert_eq!(AudioFormat::WAV.as_str(), "wav");
    assert_eq!(AudioFormat::FLAC.as_str(), "flac");
    assert_eq!(AudioFormat::OGG.as_str(), "ogg");
}

#[test]
fn test_audio_format_from_str() {
    assert_eq!(AudioFormat::from_str("mp3"), Some(AudioFormat::MP3));
    assert_eq!(AudioFormat::from_str("wav"), Some(AudioFormat::WAV));
    assert_eq!(AudioFormat::from_str("flac"), Some(AudioFormat::FLAC));
    assert_eq!(AudioFormat::from_str("ogg"), Some(AudioFormat::OGG));
    assert_eq!(AudioFormat::from_str("invalid"), None);
}

#[test]
fn test_audio_format_mime_type() {
    assert_eq!(AudioFormat::MP3.mime_type(), "audio/mpeg");
    assert_eq!(AudioFormat::WAV.mime_type(), "audio/wav");
    assert_eq!(AudioFormat::FLAC.mime_type(), "audio/flac");
    assert_eq!(AudioFormat::OGG.mime_type(), "audio/ogg");
}

#[test]
fn test_speech_synthesis_request_serialization() {
    let request = SpeechSynthesisRequest {
        tenant_id: 100_001,
        organization_id: 0,
        user_id: 100,
        text: "Hello, world!".to_string(),
        text_format: TextFormat::Plain,
        language: Some("en".to_string()),
        voice_id: Some("voice-123".to_string()),
        speed: 1.0,
        pitch: 1.0,
        volume: 1.0,
        emotion: Some("neutral".to_string()),
        emotion_intensity: 0.5,
        audio_format: AudioFormat::MP3,
        sample_rate: 24000,
        idempotency_key: Some("key-123".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: SpeechSynthesisRequest = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.tenant_id, 1);
    assert_eq!(deserialized.text, "Hello, world!");
    assert_eq!(deserialized.text_format, TextFormat::Plain);
    assert_eq!(deserialized.language, Some("en".to_string()));
    assert_eq!(deserialized.voice_id, Some("voice-123".to_string()));
    assert_eq!(deserialized.speed, 1.0);
    assert_eq!(deserialized.audio_format, AudioFormat::MP3);
    assert_eq!(deserialized.sample_rate, 24000);
}

#[test]
fn test_speech_synthesis_response_serialization() {
    let response = SpeechSynthesisResponse {
        task_id: "1".to_string(),
        task_no: "uuid-123".to_string(),
        status: "queued".to_string(),
        estimated_duration_ms: Some(2000),
    };

    let json = serde_json::to_string(&response).unwrap();
    let deserialized: SpeechSynthesisResponse = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.task_id, "1");
    assert_eq!(deserialized.task_no, "uuid-123");
    assert_eq!(deserialized.status, "queued");
    assert_eq!(deserialized.estimated_duration_ms, Some(2000));
}

#[test]
fn test_speech_synthesis_result_serialization() {
    let result = SpeechSynthesisResult {
        task_id: "1".to_string(),
        task_no: "uuid-123".to_string(),
        status: "succeeded".to_string(),
        audio_url: Some("https://example.com/audio.mp3".to_string()),
        duration_ms: Some(2500),
        file_size_bytes: Some(48000),
        mime_type: Some("audio/mpeg".to_string()),
    };

    let json = serde_json::to_string(&result).unwrap();
    let deserialized: SpeechSynthesisResult = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.task_id, "1");
    assert_eq!(deserialized.task_no, "uuid-123");
    assert_eq!(deserialized.status, "succeeded");
    assert_eq!(deserialized.audio_url, Some("https://example.com/audio.mp3".to_string()));
    assert_eq!(deserialized.duration_ms, Some(2500));
    assert_eq!(deserialized.file_size_bytes, Some(48000));
    assert_eq!(deserialized.mime_type, Some("audio/mpeg".to_string()));
}

#[test]
fn test_voice_info_serialization() {
    let voice = VoiceInfo {
        voice_id: "1".to_string(),
        voice_no: "uuid-456".to_string(),
        name: "English Male".to_string(),
        description: Some("A clear male voice".to_string()),
        language: "en".to_string(),
        gender: Some("male".to_string()),
        voice_type: "prebuilt".to_string(),
        preview_url: Some("https://example.com/preview.mp3".to_string()),
    };

    let json = serde_json::to_string(&voice).unwrap();
    let deserialized: VoiceInfo = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.voice_id, "1");
    assert_eq!(deserialized.name, "English Male");
    assert_eq!(deserialized.language, "en");
    assert_eq!(deserialized.gender, Some("male".to_string()));
}

#[test]
fn test_voice_list_request_serialization() {
    let request = VoiceListRequest {
        tenant_id: 100_001,
        user_id: Some(100),
        language: Some("en".to_string()),
        gender: Some("male".to_string()),
        voice_type: Some("prebuilt".to_string()),
        limit: 20,
        offset: 0,
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: VoiceListRequest = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.tenant_id, 1);
    assert_eq!(deserialized.user_id, Some(100));
    assert_eq!(deserialized.language, Some("en".to_string()));
    assert_eq!(deserialized.limit, 20);
    assert_eq!(deserialized.offset, 0);
}

#[test]
fn test_voice_list_response_serialization() {
    let response = VoiceListResponse {
        voices: vec![VoiceInfo {
            voice_id: "1".to_string(),
            voice_no: "uuid-456".to_string(),
            name: "English Male".to_string(),
            description: None,
            language: "en".to_string(),
            gender: None,
            voice_type: "prebuilt".to_string(),
            preview_url: None,
        }],
        total: 1,
        has_more: false,
        next_cursor: None,
    };

    let json = serde_json::to_string(&response).unwrap();
    let deserialized: VoiceListResponse = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.voices.len(), 1);
    assert_eq!(deserialized.total, 1);
    assert_eq!(deserialized.has_more, false);
}

#[test]
fn test_speech_synthesis_request_with_optional_fields() {
    let request = SpeechSynthesisRequest {
        tenant_id: 100_001,
        organization_id: 0,
        user_id: 100,
        text: "Hello".to_string(),
        text_format: TextFormat::Plain,
        language: None,
        voice_id: None,
        speed: 1.0,
        pitch: 1.0,
        volume: 1.0,
        emotion: None,
        emotion_intensity: 0.5,
        audio_format: AudioFormat::MP3,
        sample_rate: 24000,
        idempotency_key: None,
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: SpeechSynthesisRequest = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.language, None);
    assert_eq!(deserialized.voice_id, None);
    assert_eq!(deserialized.emotion, None);
    assert_eq!(deserialized.idempotency_key, None);
}
