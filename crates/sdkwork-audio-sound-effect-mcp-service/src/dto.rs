use crate::McpToolError;
use base64::{engine::general_purpose::STANDARD, Engine};
use rmcp::schemars::JsonSchema;
use sdkwork_audio_sound_effect_generation_service::{
    SoundEffectGenerationCommand, SoundEffectProviderSubmission, SoundEffectVendorId,
    SoundEffectVendorParameters,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VendorParametersInput {
    pub schema: String,
    pub values: Value,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSoundEffectInput {
    pub vendor: String,
    #[serde(default)]
    pub model: Option<String>,
    pub description: String,
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default = "default_intensity")]
    pub intensity: f64,
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_sample_rate")]
    pub sample_rate: u32,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub vendor_parameters: Option<VendorParametersInput>,
}
#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundEffectGenerationResult {
    pub vendor: String,
    pub model: Option<String>,
    pub audio_base64: String,
    pub mime_type: String,
    pub duration_ms: u64,
    pub sample_rate: u32,
    pub channels: u32,
}

impl TryFrom<GenerateSoundEffectInput> for SoundEffectGenerationCommand {
    type Error = McpToolError;
    fn try_from(input: GenerateSoundEffectInput) -> Result<Self, Self::Error> {
        Ok(Self {
            vendor: SoundEffectVendorId::new(input.vendor).map_err(McpToolError::from)?,
            model: input.model,
            description: input.description,
            duration_ms: input.duration_ms,
            style: input.style,
            intensity: input.intensity,
            format: input.format,
            sample_rate: input.sample_rate,
            idempotency_key: input.idempotency_key,
            vendor_parameters: input.vendor_parameters.map(|parameters| {
                SoundEffectVendorParameters {
                    schema: parameters.schema,
                    values: parameters.values,
                }
            }),
        })
    }
}
impl From<&SoundEffectProviderSubmission> for SoundEffectGenerationResult {
    fn from(submission: &SoundEffectProviderSubmission) -> Self {
        Self {
            vendor: submission.vendor.clone(),
            model: submission.model.clone(),
            audio_base64: STANDARD.encode(&submission.output.audio_data),
            mime_type: submission.output.mime_type.clone(),
            duration_ms: submission.output.duration_ms,
            sample_rate: submission.output.sample_rate,
            channels: submission.output.channels,
        }
    }
}
fn default_intensity() -> f64 {
    0.5
}
fn default_format() -> String {
    "wav".into()
}
fn default_sample_rate() -> u32 {
    44_100
}
