# SDKWork Sound-Effect Generation MCP Service

Provider-neutral MCP adapter for `SoundEffectGenerationServicePort`.

- Tools: `sound_effect.generate`, `sound_effect.capabilities`
- Resources: `sdkwork://sound-effect/generation/capabilities`, `sdkwork://sound-effect/generation/vendors`
- Prompt: `sound_effect.generation.request`
- Transports: stdio and MCP Streamable HTTP with SSE delivery

Sound-effect generation is currently synchronous, so the MCP surface intentionally does not expose
task retrieval or cancellation. The mounting composition root owns authentication, authorization,
origin validation, limits, tracing, listener binding, and graceful shutdown.
