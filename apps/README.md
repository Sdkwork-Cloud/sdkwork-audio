# SDKWork Audio - Application Roots

This directory contains independently runnable application roots and app surfaces.

## Structure

```
apps/
├── README.md
└── <app-surface-root>/
    ├── README.md
    ├── sdkwork.app.config.json
    ├── src/ | lib/ | App/ | entry/
    ├── packages/
    └── config/
```

## Purpose

- Application shells and surfaces
- Runnable demos and examples
- Deployable application compositions

## Related Specs

- `../sdkwork-specs/APPLICATION_SPEC.md`: Application modularization rules.
- `../sdkwork-specs/APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`: Cross-client architecture alignment.
- `../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`: PC application root architecture.
- `../sdkwork-specs/APP_H5_ARCHITECTURE_SPEC.md`: H5/Capacitor application root architecture.

## Verification

- Application build and test
- SDK integration verification
