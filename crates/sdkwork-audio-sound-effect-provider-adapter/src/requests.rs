use sdkwork_audio_ai_engine_rust::SoundEffectRequest;
use sdkwork_audio_sound_effect_provider_spi::{
    SoundEffectGenerationCommand, SoundEffectProviderError, SoundEffectProviderResult,
};

#[derive(Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct AudioEngineVendorParameters {}

pub fn build_audio_engine_sound_effect_request(
    command: &SoundEffectGenerationCommand,
) -> SoundEffectProviderResult<SoundEffectRequest> {
    command.validate()?;
    let _: AudioEngineVendorParameters =
        decode_vendor_parameters(command, "audio-engine.sound-effect-generation.v1")?;
    Ok(SoundEffectRequest {
        description: command.description.trim().to_string(),
        duration_ms: command.duration_ms,
        style: command.style.clone(),
        intensity: command.intensity,
        format: command.format.trim().to_ascii_lowercase(),
        sample_rate: command.sample_rate,
    })
}

fn decode_vendor_parameters<T>(
    command: &SoundEffectGenerationCommand,
    expected_schema: &str,
) -> SoundEffectProviderResult<T>
where
    T: serde::de::DeserializeOwned + Default,
{
    let Some(parameters) = command.vendor_parameters.as_ref() else {
        return Ok(T::default());
    };
    if parameters.schema.trim() != expected_schema {
        return Err(SoundEffectProviderError::UnsupportedParameter(format!(
            "vendor parameter schema {} is not valid for {}",
            parameters.schema, command.vendor
        )));
    }
    serde_json::from_value(parameters.values.clone()).map_err(|error| {
        SoundEffectProviderError::InvalidRequest(format!(
            "invalid {} vendor parameters: {error}",
            command.vendor
        ))
    })
}
