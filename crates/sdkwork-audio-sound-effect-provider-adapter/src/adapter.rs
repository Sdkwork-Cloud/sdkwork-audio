use std::sync::Arc;

use sdkwork_audio_ai_engine_rust::AudioAiEngine;
use sdkwork_audio_sound_effect_provider_spi::{
    GeneratedSoundEffect, SoundEffectGenerationCommand, SoundEffectProvider,
    SoundEffectProviderDescriptor, SoundEffectProviderError, SoundEffectProviderResult,
    SoundEffectProviderSubmission, SoundEffectVendorId,
};

use crate::requests::build_audio_engine_sound_effect_request;

pub const SOUND_EFFECT_PROVIDER_ADAPTER_ID: &str = "sdkwork-audio-sound-effect-provider-adapter";

pub struct AudioEngineSoundEffectProviderAdapter {
    engine: Arc<dyn AudioAiEngine + Send + Sync>,
    descriptor: SoundEffectProviderDescriptor,
}

impl AudioEngineSoundEffectProviderAdapter {
    pub fn new(engine: Arc<dyn AudioAiEngine + Send + Sync>) -> Self {
        Self::with_provider_id(SOUND_EFFECT_PROVIDER_ADAPTER_ID, engine)
    }

    pub fn with_provider_id(
        provider_id: impl Into<String>,
        engine: Arc<dyn AudioAiEngine + Send + Sync>,
    ) -> Self {
        let vendor = engine.engine_type().as_str();
        Self {
            engine,
            descriptor: SoundEffectProviderDescriptor {
                id: provider_id.into(),
                vendors: vec![SoundEffectVendorId::new(vendor).expect("audio engine vendor")],
            },
        }
    }
}

#[async_trait::async_trait]
impl SoundEffectProvider for AudioEngineSoundEffectProviderAdapter {
    fn descriptor(&self) -> &SoundEffectProviderDescriptor {
        &self.descriptor
    }

    fn validate(&self, command: &SoundEffectGenerationCommand) -> SoundEffectProviderResult<()> {
        if !self.descriptor.supports_vendor(&command.vendor) {
            return Err(SoundEffectProviderError::UnsupportedVendor(
                command.vendor.to_string(),
            ));
        }
        build_audio_engine_sound_effect_request(command)?;
        Ok(())
    }

    async fn generate(
        &self,
        command: &SoundEffectGenerationCommand,
    ) -> SoundEffectProviderResult<SoundEffectProviderSubmission> {
        self.validate(command)?;
        let request = build_audio_engine_sound_effect_request(command)?;
        let result = self
            .engine
            .generate_sound_effect(request)
            .await
            .map_err(|error| SoundEffectProviderError::ProviderFailure(error.to_string()))?;
        Ok(SoundEffectProviderSubmission {
            provider_id: self.descriptor.id.clone(),
            vendor: command.vendor.to_string(),
            model: command.model.clone(),
            output: GeneratedSoundEffect {
                audio_data: result.audio_data,
                mime_type: result.mime_type,
                duration_ms: result.duration_ms,
                sample_rate: result.sample_rate,
                channels: result.channels,
            },
        })
    }
}
