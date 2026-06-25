# SDKWork Audio - API Contracts

This directory contains API contracts and API source inputs for the Audio application.

## Structure

```
apis/
├── open-api/audio/          # Open API contracts
├── app-api/audio/           # App API contracts
├── backend-api/audio/       # Backend API contracts
├── rpc/                     # RPC contracts
├── async/                   # Async/event API contracts
├── internal/                # Internal API contracts
├── examples/                # API examples
├── changelogs/              # API changelogs
└── tests/                   # API validation tests
```

## Purpose

- API contracts for all API kinds (HTTP, RPC, async/event)
- API examples and changelogs
- API validation inputs

## Related Specs

- `../sdkwork-specs/API_SPEC.md`: HTTP API contract rules.
- `../sdkwork-specs/RPC_SPEC.md`: RPC/gRPC contract rules.
- `../sdkwork-specs/EVENT_SPEC.md`: Async/event API rules.

## Verification

- API validation tests
- SDK generation verification
