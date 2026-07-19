# SDKWork Audio Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-06-24
Specs: ARCHITECTURE_DECISION_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- [TECH-alignment-report.md](TECH-alignment-report.md)
- [TECH-api-design.md](TECH-api-design.md)
- [TECH-audio-app-design-report.md](TECH-audio-app-design-report.md)
- [TECH-core-services-summary.md](TECH-core-services-summary.md)
- [TECH-database-design.md](TECH-database-design.md)
- [TECH-final-summary.md](TECH-final-summary.md)
- [TECH-project-completion-report.md](TECH-project-completion-report.md)
- [TECH-sdk-design.md](TECH-sdk-design.md)
- [TECH-sound-effect-service-implementation.md](TECH-sound-effect-service-implementation.md)
- [TECH-speech-service-implementation.md](TECH-speech-service-implementation.md)
- [TECH-transcription-service-implementation.md](TECH-transcription-service-implementation.md)
- [TECH-translation-service-implementation.md](TECH-translation-service-implementation.md)

## 1. Architecture Overview

Architecture detail lives in the linked TECH shards below.


## 2. Technology Choices

## 3. System Boundaries And Modules

Sound-effect generation follows the standard provider boundary:

- L2 `sdkwork-audio-sound-effect-generation-service` is the application-facing entrypoint.
- L3 `sdkwork-audio-sound-effect-provider-spi` owns transport-neutral contracts and provider selection.
- L4 `sdkwork-audio-sound-effect-provider-adapter` converts the SPI to the existing `AudioAiEngine` port.
- `sdkwork-audio-sound-effect-mcp-service` is the agent-facing protocol adapter. It depends only on
  `SoundEffectGenerationServicePort` and owns `sound_effect.*` tools, resources, prompts, stdio,
  and Streamable HTTP/SSE construction.
- Provider engines, SDK routes, DTO conversion, and provider errors never leak into L2 or L3.

See [ADR-20260719-sound-effect-provider-spi.md](../decisions/ADR-20260719-sound-effect-provider-spi.md).

## 4. Directory And Package Layout

```text
crates/sdkwork-audio-sound-effect-provider-spi/
crates/sdkwork-audio-sound-effect-generation-service/
crates/sdkwork-audio-sound-effect-provider-adapter/
crates/sdkwork-audio-sound-effect-mcp-service/
```

## 5. API, SDK, And Data Ownership

## 6. Security, Privacy, And Observability

## 7. Deployment And Runtime Topology

Sound-effect MCP generation is synchronous. Hosts choose stdio or Streamable HTTP; SSE is the
Streamable HTTP response channel, not a separate compatibility endpoint. The host owns security,
limits, observability, listener lifecycle, and graceful shutdown.

## 8. Architecture Decision Index

## 9. Verification

- `cargo test -p sdkwork-audio-sound-effect-mcp-service`
- `cargo clippy -p sdkwork-audio-sound-effect-mcp-service --all-targets -- -D warnings`
