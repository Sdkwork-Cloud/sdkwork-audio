use crate::SoundEffectProviderError;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct SoundEffectVendorId(String);

impl SoundEffectVendorId {
    pub fn new(value: impl Into<String>) -> Result<Self, SoundEffectProviderError> {
        let value = value.into().trim().to_ascii_lowercase().replace('_', "-");
        if value.is_empty() {
            return Err(SoundEffectProviderError::InvalidRequest(
                "vendor is required".to_string(),
            ));
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        {
            return Err(SoundEffectProviderError::InvalidRequest(
                "vendor must use lowercase letters, digits, or hyphens".to_string(),
            ));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SoundEffectVendorId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SoundEffectVendorParameters {
    pub schema: String,
    pub values: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoundEffectGenerationCommand {
    pub vendor: SoundEffectVendorId,
    pub model: Option<String>,
    pub description: String,
    pub duration_ms: Option<u64>,
    pub style: Option<String>,
    pub intensity: f64,
    pub format: String,
    pub sample_rate: u32,
    pub idempotency_key: Option<String>,
    pub vendor_parameters: Option<SoundEffectVendorParameters>,
}

impl SoundEffectGenerationCommand {
    pub fn validate(&self) -> Result<(), SoundEffectProviderError> {
        if self.description.trim().is_empty() {
            return Err(SoundEffectProviderError::InvalidRequest(
                "description is required".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&self.intensity) {
            return Err(SoundEffectProviderError::InvalidRequest(
                "intensity must be between 0.0 and 1.0".to_string(),
            ));
        }
        if self.sample_rate == 0 {
            return Err(SoundEffectProviderError::InvalidRequest(
                "sample_rate must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedSoundEffect {
    pub audio_data: Vec<u8>,
    pub mime_type: String,
    pub duration_ms: u64,
    pub sample_rate: u32,
    pub channels: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoundEffectProviderSubmission {
    pub provider_id: String,
    pub vendor: String,
    pub model: Option<String>,
    pub output: GeneratedSoundEffect,
}
