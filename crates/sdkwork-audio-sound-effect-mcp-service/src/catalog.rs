use rmcp::model::{
    GetPromptResult, ListPromptsResult, ListResourcesResult, Prompt, PromptMessage, Resource,
    ResourceContents, Role,
};
use sdkwork_audio_sound_effect_generation_service::SoundEffectProviderDescriptor;
pub(crate) const CAPABILITIES_URI: &str = "sdkwork://sound-effect/generation/capabilities";
pub(crate) const VENDORS_URI: &str = "sdkwork://sound-effect/generation/vendors";
pub(crate) const GENERATION_PROMPT: &str = "sound_effect.generation.request";
pub(crate) fn resources() -> ListResourcesResult {
    ListResourcesResult::with_all_items(vec![
        Resource::new(CAPABILITIES_URI, "sound-effect-generation-capabilities")
            .with_title("Sound-effect generation capabilities")
            .with_mime_type("application/json"),
        Resource::new(VENDORS_URI, "sound-effect-generation-vendors")
            .with_title("Sound-effect generation vendors")
            .with_mime_type("application/json"),
    ])
}
pub(crate) fn catalog(descriptors: Vec<SoundEffectProviderDescriptor>) -> serde_json::Value {
    let providers = descriptors.into_iter().map(|descriptor| serde_json::json!({"vendors":descriptor.vendors.into_iter().map(|vendor|vendor.to_string()).collect::<Vec<_>>() })).collect::<Vec<_>>();
    serde_json::json!({"domain":"sound-effect","tools":["sound_effect.generate","sound_effect.capabilities"],"transports":["stdio","streamable-http-sse"],"providers":providers})
}
pub(crate) fn read(
    uri: &str,
    descriptors: Vec<SoundEffectProviderDescriptor>,
) -> Option<ResourceContents> {
    let catalog = catalog(descriptors);
    let value = match uri {
        CAPABILITIES_URI => catalog,
        VENDORS_URI => catalog.get("providers")?.clone(),
        _ => return None,
    };
    Some(
        ResourceContents::text(serde_json::to_string_pretty(&value).ok()?, uri)
            .with_mime_type("application/json"),
    )
}
pub(crate) fn prompts() -> ListPromptsResult {
    ListPromptsResult::with_all_items(vec![Prompt::new(
        GENERATION_PROMPT,
        Some(
            "Prepare a provider-neutral sound-effect generation request for sound_effect.generate.",
        ),
        None,
    )])
}
pub(crate) fn prompt() -> GetPromptResult {
    GetPromptResult::new(vec![PromptMessage::new_text(Role::User, "Create a sound-effect generation request. Inspect sdkwork://sound-effect/generation/vendors, choose duration, intensity, format, and sample rate, keep provider-only fields inside vendorParameters with its schema identifier, and invoke sound_effect.generate.")]).with_description("Provider-neutral sound-effect generation request workflow")
}
