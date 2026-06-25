# SDKWork Audio Backend SDK

SDK family for SDKWork Audio Backend API.

## Overview

This SDK family provides client libraries for the SDKWork Audio Backend API, which includes:
- Provider route management
- Task administration
- Webhook management
- Usage analytics
- Content moderation
- System management

## SDK Languages

| Language | Package | Status |
|----------|---------|--------|
| TypeScript | `@sdkwork/audio-backend-sdk` | Materialized |
| Dart | `sdkwork_audio_backend_sdk` | Materialized |
| Python | `sdkwork-audio-backend-sdk` | Materialized |
| Go | `github.com/sdkwork/sdkwork-audio-backend-sdk` | Materialized |
| Java | `com.sdkwork:sdkwork-audio-backend-sdk` | Materialized |
| Kotlin | `com.sdkwork:sdkwork-audio-backend-sdk` | Materialized |
| Swift | `sdkwork-audio-backend-sdk` | Materialized |
| C# | `SDKWork.Audio.BackendSdk` | Materialized |
| Flutter | `sdkwork_audio_backend_sdk` | Materialized |
| Rust | `sdkwork-audio-backend-sdk` | Materialized |
| PHP | `sdkwork/audio-backend-sdk` | Materialized |
| Ruby | `sdkwork-audio-backend-sdk` | Materialized |

## API Authority

- **Authority**: `sdkwork-audio-backend-api`
- **Prefix**: `/backend/v3/api/audio`
- **OpenAPI**: `openapi/sdkwork-audio-backend-api.openapi.yaml`

## SDK Dependencies

- `sdkwork-iam-sdk`: Authentication capability

## Generation

```powershell
# Generate TypeScript SDK
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\generate-sdk.ps1 -Languages typescript
```

## Verification

```powershell
# Verify generated SDK
node .\bin\publish-core.mjs --language typescript --project-dir . --action check
node .\bin\publish-core.mjs --language typescript --project-dir . --action build
```

## Related Specs

- `../sdkwork-specs/SDK_SPEC.md`: SDK generation and integration rules.
- `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`: SDK workspace layout rules.
- `../sdkwork-specs/API_SPEC.md`: HTTP API contract rules.

---

*SDK family maintained by SDKWork Audio SDK System*
*Version: 1.0.0*
*Date: June 14, 2026*
