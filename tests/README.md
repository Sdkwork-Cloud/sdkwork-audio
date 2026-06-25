# SDKWork Audio - Tests

This directory contains cross-package tests, contract tests, integration tests, and end-to-end tests.

## Structure

```
tests/
├── README.md
├── contract/           # Contract tests
├── integration/        # Integration tests
├── e2e/                # End-to-end tests
├── fixtures/           # Test fixtures
└── static/             # Static verification inputs
```

## Purpose

- Cross-package verification
- Contract tests for API and SDK
- Integration tests for system behavior
- End-to-end tests for user workflows

## Related Specs

- `../sdkwork-specs/TEST_SPEC.md`: Verification and contract testing expectations.
- `../sdkwork-specs/QUALITY_GATE_SPEC.md`: Quality gate definitions.

## Verification

- `pnpm test`
- `pnpm test:rust`
