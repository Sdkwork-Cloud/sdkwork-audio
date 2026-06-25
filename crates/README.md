# SDKWork Audio - Rust Crates

This directory contains Rust crates for the Audio application.

## Structure

```
crates/
├── README.md
├── sdkwork-router-audio-app-api/        # App API route definitions
├── sdkwork-router-audio-backend-api/    # Backend API route definitions
├── sdkwork-audio-artifact-drive-service/ # Drive integration
├── sdkwork-audio-generation-repository-sqlx/ # Database storage
├── sdkwork-audio-realtime-rust/         # Real-time processing
└── sdkwork-audio-ai-engine-rust/        # AI engine integration
```

## Purpose

- Route crates for API definitions
- Service crates for business logic
- Repository crates for data access
- Worker crates for background jobs

## Related Specs

- `../sdkwork-specs/RUST_CODE_SPEC.md`: Rust crate and module structure rules.
- `../sdkwork-specs/RUST_RPC_SPEC.md`: Rust RPC implementation rules.
- `../sdkwork-specs/WEB_BACKEND_SPEC.md`: Web backend implementation rules.

## Verification

- `cargo test --workspace`
- `cargo clippy --workspace`
