use async_trait::async_trait;

use crate::{
    SoundEffectGenerationCommand, SoundEffectProviderResult, SoundEffectProviderSubmission,
    SoundEffectVendorId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SoundEffectProviderDescriptor {
    pub id: String,
    pub vendors: Vec<SoundEffectVendorId>,
}

impl SoundEffectProviderDescriptor {
    pub fn supports_vendor(&self, vendor: &SoundEffectVendorId) -> bool {
        self.vendors.iter().any(|candidate| candidate == vendor)
    }
}

#[async_trait]
pub trait SoundEffectProvider: Send + Sync {
    fn descriptor(&self) -> &SoundEffectProviderDescriptor;
    fn validate(&self, command: &SoundEffectGenerationCommand) -> SoundEffectProviderResult<()>;
    async fn generate(
        &self,
        command: &SoundEffectGenerationCommand,
    ) -> SoundEffectProviderResult<SoundEffectProviderSubmission>;
}
