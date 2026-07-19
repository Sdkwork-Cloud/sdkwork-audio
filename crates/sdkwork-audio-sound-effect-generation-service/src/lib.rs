//! Unified application service for sound-effect generation providers.

mod service;

pub use sdkwork_audio_sound_effect_provider_spi::*;
pub use service::{SoundEffectGenerationService, SoundEffectGenerationServicePort};
