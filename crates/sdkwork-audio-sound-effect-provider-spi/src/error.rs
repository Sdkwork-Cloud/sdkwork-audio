#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SoundEffectProviderError {
    #[error("sound-effect request is invalid: {0}")]
    InvalidRequest(String),
    #[error("sound-effect vendor is unsupported: {0}")]
    UnsupportedVendor(String),
    #[error("sound-effect capability is unsupported: {0}")]
    UnsupportedCapability(String),
    #[error("sound-effect parameter is unsupported: {0}")]
    UnsupportedParameter(String),
    #[error("sound-effect provider is not configured: {0}")]
    ProviderNotConfigured(String),
    #[error("sound-effect provider is unavailable: {0}")]
    ProviderUnavailable(String),
    #[error("sound-effect provider failed: {0}")]
    ProviderFailure(String),
    #[error("sound-effect provider configuration is invalid: {0}")]
    Configuration(String),
}

pub type SoundEffectProviderResult<T> = Result<T, SoundEffectProviderError>;
