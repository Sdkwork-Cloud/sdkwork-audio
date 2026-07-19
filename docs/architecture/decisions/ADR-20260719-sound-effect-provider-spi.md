# ADR-20260719: Sound-Effect Generation Provider SPI

Status: accepted
Date: 2026-07-19
Owner: SDKWork Audio maintainers

## Context

Sound-effect callers need one stable generation entrypoint while concrete engines may expose
different models, request fields, result shapes, and transport mechanisms. The generated Rust SDK
currently has no typed sound-effect generation route, so an SDK-backed adapter cannot be added
without inventing an unsupported raw HTTP contract.

## Decision

Use `sdkwork-audio-sound-effect-generation-service` as the public L2 entrypoint,
`sdkwork-audio-sound-effect-provider-spi` as the transport-neutral L3 contract, and
`sdkwork-audio-sound-effect-provider-adapter` as the L4 owner of the existing `AudioAiEngine`
conversion.

The SPI owns stable vendor, model, common request, versioned vendor-parameter, dispatch, and
normalized result contracts. Engine request conversion and engine errors remain in the adapter.
ClawRouter is not a public vendor or provider identity and is not a dependency of this capability,
because no typed generated SDK route exists.

A future typed SDK adapter may replace or coexist with the engine adapter through the same SPI and
registry without changing callers of the L2 service.

## Consequences

- Application and domain callers depend only on the unified generation service port.
- Provider-specific fields are isolated behind explicit, versioned schemas.
- The existing audio engine remains reusable without leaking its contract into L2 or L3.
- A missing SDK operation fails as an unsupported capability instead of falling back to raw HTTP.

## Verification

Run the SPI, adapter, and unified service tests, then validate component port bindings and
application layering for the audio workspace.
