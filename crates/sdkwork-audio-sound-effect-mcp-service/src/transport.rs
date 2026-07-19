use crate::SoundEffectGenerationMcpService;
use rmcp::{
    service::{RunningService, ServerInitializeError},
    transport::streamable_http_server::{
        session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
    },
    RoleServer, ServiceExt,
};
use std::sync::Arc;
pub type SoundEffectGenerationMcpHttpService =
    StreamableHttpService<SoundEffectGenerationMcpService, LocalSessionManager>;
pub fn streamable_http_service(
    service: SoundEffectGenerationMcpService,
    config: StreamableHttpServerConfig,
) -> SoundEffectGenerationMcpHttpService {
    StreamableHttpService::new(
        move || Ok(service.clone()),
        Arc::new(LocalSessionManager::default()),
        config,
    )
}
pub async fn serve_stdio(
    service: SoundEffectGenerationMcpService,
) -> Result<RunningService<RoleServer, SoundEffectGenerationMcpService>, ServerInitializeError> {
    service.serve(rmcp::transport::stdio()).await
}
