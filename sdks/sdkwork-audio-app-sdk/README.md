# SDKWork Audio App SDK

SDK family for SDKWork Audio App API.

## Overview

This SDK family provides client libraries for the SDKWork Audio App API, which includes:
- Speech synthesis
- Audio transcription
- Audio translation
- Sound effect generation
- Voice management
- Real-time audio processing
- Workspace management

## SDK Languages

| Language | Package | Status |
|----------|---------|--------|
| TypeScript | `@sdkwork/audio-app-sdk` | Materialized |
| Dart | `sdkwork_audio_app_sdk` | Materialized |
| Python | `sdkwork-audio-app-sdk` | Materialized |
| Go | `github.com/sdkwork/sdkwork-audio-app-sdk` | Materialized |
| Java | `com.sdkwork:sdkwork-audio-app-sdk` | Materialized |
| Kotlin | `com.sdkwork:sdkwork-audio-app-sdk` | Materialized |
| Swift | `sdkwork-audio-app-sdk` | Materialized |
| C# | `SDKWork.Audio.AppSdk` | Materialized |
| Flutter | `sdkwork_audio_app_sdk` | Materialized |
| Rust | `sdkwork-audio-app-sdk` | Materialized |
| PHP | `sdkwork/audio-app-sdk` | Materialized |
| Ruby | `sdkwork-audio-app-sdk` | Materialized |

## API Authority

- **Authority**: `sdkwork-audio-app-api`
- **Prefix**: `/app/v3/api/audio`
- **OpenAPI**: `openapi/sdkwork-audio-app-api.openapi.yaml`

## SDK Dependencies

- `clawrouter-open-sdk`: AI audio generation provider capability
- `sdkwork-drive-sdk`: Audio storage capability
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
