use std::sync::Arc;

use sdkwork_audio_ai_engine_rust::MockAudioAiEngine;
use sdkwork_audio_sound_effect_generation_service::{
    SoundEffectGenerationCommand, SoundEffectGenerationService, SoundEffectGenerationServicePort,
    SoundEffectProviderRegistry, SoundEffectVendorId,
};
use sdkwork_audio_sound_effect_provider_adapter::{
    AudioEngineSoundEffectProviderAdapter, SOUND_EFFECT_PROVIDER_ADAPTER_ID,
};

#[tokio::test]
async fn unified_sound_effect_service_dispatches_through_engine_adapter() {
    let provider = Arc::new(AudioEngineSoundEffectProviderAdapter::new(Arc::new(
        MockAudioAiEngine,
    )));
    let registry = SoundEffectProviderRegistry::builder()
        .register(provider)
        .expect("provider")
        .default_provider(SOUND_EFFECT_PROVIDER_ADAPTER_ID)
        .build()
        .expect("registry");
    let service = SoundEffectGenerationService::new(registry);
    let submission = service
        .generate(SoundEffectGenerationCommand {
            vendor: SoundEffectVendorId::new("custom").expect("vendor"),
            model: None,
            description: "Cinematic impact".to_string(),
            duration_ms: Some(3000),
            style: Some("cinematic".to_string()),
            intensity: 0.8,
            format: "wav".to_string(),
            sample_rate: 44100,
            idempotency_key: None,
            vendor_parameters: None,
        })
        .await
        .expect("submission");
    assert_eq!(submission.provider_id, SOUND_EFFECT_PROVIDER_ADAPTER_ID);
    assert_eq!(submission.vendor, "custom");
    assert_eq!(submission.output.audio_data.len(), 1024);
}
