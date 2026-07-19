//! L4 adapter from the existing audio engine port to the sound-effect provider SPI.

mod adapter;
mod requests;

pub use adapter::{AudioEngineSoundEffectProviderAdapter, SOUND_EFFECT_PROVIDER_ADAPTER_ID};
pub use requests::build_audio_engine_sound_effect_request;
