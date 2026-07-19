use schemars::JsonSchema;
use sdkwork_audio_sound_effect_generation_service::SoundEffectProviderError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpToolError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}
impl From<SoundEffectProviderError> for McpToolError {
    fn from(error: SoundEffectProviderError) -> Self {
        let (code, retryable) = match &error {
            SoundEffectProviderError::InvalidRequest(_) => ("invalid_request", false),
            SoundEffectProviderError::UnsupportedVendor(_) => ("unsupported_vendor", false),
            SoundEffectProviderError::UnsupportedCapability(_) => ("unsupported_capability", false),
            SoundEffectProviderError::UnsupportedParameter(_) => ("unsupported_parameter", false),
            SoundEffectProviderError::ProviderNotConfigured(_) => {
                ("provider_not_configured", false)
            }
            SoundEffectProviderError::ProviderUnavailable(_) => ("provider_unavailable", true),
            SoundEffectProviderError::ProviderFailure(_) => ("provider_failure", true),
            SoundEffectProviderError::Configuration(_) => ("configuration", false),
        };
        Self {
            code: code.into(),
            message: error.to_string(),
            retryable,
        }
    }
}
