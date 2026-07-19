use crate::{GenerateSoundEffectInput, McpToolError, SoundEffectGenerationResult};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, ErrorData, GetPromptRequestParams, GetPromptResult, Implementation,
        ListPromptsResult, ListResourcesResult, PaginatedRequestParams, ReadResourceRequestParams,
        ReadResourceResult, ServerCapabilities, ServerInfo, Tool,
    },
    service::RequestContext,
    tool, tool_handler, tool_router, Json, RoleServer, ServerHandler,
};
use sdkwork_audio_sound_effect_generation_service::{
    SoundEffectGenerationServicePort, SoundEffectProviderDescriptor,
};
use std::sync::Arc;
#[derive(Clone)]
pub struct SoundEffectGenerationMcpService {
    generation_service: Arc<dyn SoundEffectGenerationServicePort>,
    tool_router: ToolRouter<Self>,
}
impl SoundEffectGenerationMcpService {
    pub fn new(generation_service: Arc<dyn SoundEffectGenerationServicePort>) -> Self {
        Self {
            generation_service,
            tool_router: Self::tool_router(),
        }
    }
    pub fn tools(&self) -> Vec<Tool> {
        self.tool_router.list_all()
    }
    pub fn provider_descriptors(&self) -> Vec<SoundEffectProviderDescriptor> {
        self.generation_service.provider_descriptors()
    }
}
#[tool_router]
impl SoundEffectGenerationMcpService {
    #[tool(
        name = "sound_effect.generate",
        description = "Generate a sound effect through the unified sound-effect generation service."
    )]
    async fn generate(
        &self,
        Parameters(input): Parameters<GenerateSoundEffectInput>,
    ) -> Result<Json<SoundEffectGenerationResult>, Json<McpToolError>> {
        let submission = self
            .generation_service
            .generate(input.try_into().map_err(Json)?)
            .await
            .map_err(|error| Json(error.into()))?;
        Ok(Json((&submission).into()))
    }
    #[tool(
        name = "sound_effect.capabilities",
        description = "List registered sound-effect generation vendors."
    )]
    async fn capabilities(&self) -> CallToolResult {
        CallToolResult::structured(crate::catalog::catalog(self.provider_descriptors()))
    }
}
#[tool_handler(router = self.tool_router)]
impl ServerHandler for SoundEffectGenerationMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().enable_resources().enable_prompts().build()).with_server_info(Implementation::new("sdkwork-audio-sound-effect-mcp-service", env!("CARGO_PKG_VERSION"))).with_instructions("Use provider-neutral sound-effect generation tools and inspect vendor resources before setting vendor-specific parameters.")
    }
    async fn list_resources(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        Ok(crate::catalog::resources())
    }
    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        crate::catalog::read(&request.uri, self.provider_descriptors())
            .map(|content| ReadResourceResult::new(vec![content]))
            .ok_or_else(|| {
                ErrorData::resource_not_found("sound-effect MCP resource was not found", None)
            })
    }
    async fn list_prompts(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        Ok(crate::catalog::prompts())
    }
    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        if request.name == crate::catalog::GENERATION_PROMPT {
            Ok(crate::catalog::prompt())
        } else {
            Err(ErrorData::invalid_params(
                "sound-effect MCP prompt was not found",
                None,
            ))
        }
    }
}
