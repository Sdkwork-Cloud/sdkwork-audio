use std::sync::Arc;

use async_trait::async_trait;
use sdkwork_audio_sound_effect_provider_spi::{
    SoundEffectGenerationCommand, SoundEffectProviderDescriptor, SoundEffectProviderRegistry,
    SoundEffectProviderResult, SoundEffectProviderSubmission,
};

#[async_trait]
pub trait SoundEffectGenerationServicePort: Send + Sync {
    async fn generate(
        &self,
        command: SoundEffectGenerationCommand,
    ) -> SoundEffectProviderResult<SoundEffectProviderSubmission>;
    fn provider_descriptors(&self) -> Vec<SoundEffectProviderDescriptor>;
}

#[derive(Clone)]
pub struct SoundEffectGenerationService {
    providers: Arc<SoundEffectProviderRegistry>,
}

impl SoundEffectGenerationService {
    pub fn new(providers: SoundEffectProviderRegistry) -> Self {
        Self {
            providers: Arc::new(providers),
        }
    }
}

#[async_trait]
impl SoundEffectGenerationServicePort for SoundEffectGenerationService {
    async fn generate(
        &self,
        command: SoundEffectGenerationCommand,
    ) -> SoundEffectProviderResult<SoundEffectProviderSubmission> {
        let provider = self.providers.select_for_vendor(&command.vendor)?;
        provider.validate(&command)?;
        provider.generate(&command).await
    }

    fn provider_descriptors(&self) -> Vec<SoundEffectProviderDescriptor> {
        self.providers.descriptors()
    }
}
