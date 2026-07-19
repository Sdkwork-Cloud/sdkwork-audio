use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{
    SoundEffectProvider, SoundEffectProviderError, SoundEffectProviderResult, SoundEffectVendorId,
};

#[derive(Clone, Default)]
pub struct SoundEffectProviderRegistry {
    providers: BTreeMap<String, Arc<dyn SoundEffectProvider>>,
    default_provider_id: Option<String>,
}

impl SoundEffectProviderRegistry {
    pub fn builder() -> SoundEffectProviderRegistryBuilder {
        SoundEffectProviderRegistryBuilder::default()
    }
    pub fn select_for_vendor(
        &self,
        vendor: &SoundEffectVendorId,
    ) -> SoundEffectProviderResult<Arc<dyn SoundEffectProvider>> {
        if let Some(provider) = self
            .default_provider_id
            .as_deref()
            .and_then(|id| self.providers.get(id))
            .filter(|provider| provider.descriptor().supports_vendor(vendor))
        {
            return Ok(provider.clone());
        }
        self.providers
            .values()
            .find(|provider| provider.descriptor().supports_vendor(vendor))
            .cloned()
            .ok_or_else(|| SoundEffectProviderError::UnsupportedVendor(vendor.to_string()))
    }
    pub fn descriptors(&self) -> Vec<crate::SoundEffectProviderDescriptor> {
        self.providers
            .values()
            .map(|provider| provider.descriptor().clone())
            .collect()
    }
}

#[derive(Default)]
pub struct SoundEffectProviderRegistryBuilder {
    providers: BTreeMap<String, Arc<dyn SoundEffectProvider>>,
    default_provider_id: Option<String>,
}

impl SoundEffectProviderRegistryBuilder {
    pub fn register(
        mut self,
        provider: Arc<dyn SoundEffectProvider>,
    ) -> SoundEffectProviderResult<Self> {
        let id = provider.descriptor().id.trim().to_string();
        if id.is_empty() {
            return Err(SoundEffectProviderError::Configuration(
                "provider id is required".to_string(),
            ));
        }
        if self.providers.insert(id.clone(), provider).is_some() {
            return Err(SoundEffectProviderError::Configuration(format!(
                "duplicate provider id: {id}"
            )));
        }
        Ok(self)
    }
    pub fn default_provider(mut self, provider_id: impl Into<String>) -> Self {
        self.default_provider_id = Some(provider_id.into());
        self
    }
    pub fn build(self) -> SoundEffectProviderResult<SoundEffectProviderRegistry> {
        if let Some(id) = self.default_provider_id.as_deref() {
            if !self.providers.contains_key(id) {
                return Err(SoundEffectProviderError::Configuration(format!(
                    "default provider is not registered: {id}"
                )));
            }
        }
        Ok(SoundEffectProviderRegistry {
            providers: self.providers,
            default_provider_id: self.default_provider_id,
        })
    }
}
