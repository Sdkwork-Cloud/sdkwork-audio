# SDKWork Audio - Local Specifications

This directory contains local specifications that narrow or extend root SDKWork standards for the Audio application.

## Files

- `component.spec.json`: Component specification for the Audio workspace.
- `README.md`: This file.

## Purpose

Local specifications define:
- Component boundaries and ownership
- Public exports and runtime entrypoints
- Verification commands
- SDK dependencies

## Related Specs

- `../sdkwork-specs/COMPONENT_SPEC.md`: Component discovery and authority chain rules.
- `../sdkwork-specs/CODE_STYLE_SPEC.md`: Source structure and generated code boundaries.
- `../sdkwork-specs/NAMING_SPEC.md`: Canonical SDKWork naming rules.
- `../sdkwork-specs/RUST_CODE_SPEC.md`: Rust crate and module structure rules.
- `../sdkwork-specs/DATABASE_SPEC.md`: Database schema and persistence rules.
- `../sdkwork-specs/API_SPEC.md`: HTTP API contract rules.
- `../sdkwork-specs/SDK_SPEC.md`: SDK generation and integration rules.

## Verification

- `pnpm typecheck`: TypeScript type checking
- `pnpm test:rust`: Rust tests
