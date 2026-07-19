use sdkwork_audio_sound_effect_provider_adapter::{
    build_audio_engine_sound_effect_request, SOUND_EFFECT_PROVIDER_ADAPTER_ID,
};
use sdkwork_audio_sound_effect_provider_spi::{
    SoundEffectGenerationCommand, SoundEffectVendorId, SoundEffectVendorParameters,
};

fn command() -> SoundEffectGenerationCommand {
    SoundEffectGenerationCommand {
        vendor: SoundEffectVendorId::new("custom").expect("vendor"),
        model: None,
        description: "UI confirmation".to_string(),
        duration_ms: Some(500),
        style: Some("clean".to_string()),
        intensity: 0.5,
        format: "wav".to_string(),
        sample_rate: 44100,
        idempotency_key: None,
        vendor_parameters: None,
    }
}

#[test]
fn maps_common_parameters_to_audio_engine_request() {
    let request = build_audio_engine_sound_effect_request(&command()).expect("request");
    assert_eq!(request.description, "UI confirmation");
    assert_eq!(request.duration_ms, Some(500));
    assert_eq!(request.format, "wav");
    assert_eq!(
        SOUND_EFFECT_PROVIDER_ADAPTER_ID,
        "sdkwork-audio-sound-effect-provider-adapter"
    );
}

#[test]
fn rejects_vendor_parameter_schema_mismatch() {
    let mut command = command();
    command.vendor_parameters = Some(SoundEffectVendorParameters {
        schema: "openai.speech-generation.v1".to_string(),
        values: serde_json::json!({}),
    });
    let error = build_audio_engine_sound_effect_request(&command).expect_err("schema mismatch");
    assert!(error.to_string().contains("schema"));
}
