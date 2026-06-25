# ADR-20260614-audio-app-architecture

Status: accepted
Owner: sdkwork-audio
Date: 2026-06-14
Specs: ARCHITECTURE_DECISION_SPEC.md, DATABASE_SPEC.md, API_SPEC.md, SDK_SPEC.md

## Context

The SDKWork ecosystem needs a professional-grade audio application that provides speech synthesis, audio transcription, audio translation, sound effect generation, voice cloning, and real-time audio processing capabilities. The application must follow SDKWork standards and integrate with existing platform capabilities.

Key requirements:
1. Multi-engine AI support for various audio processing tasks
2. Real-time capabilities for live transcription and translation
3. Professional audio workspace with waveform visualization
4. Multi-tenant architecture for SaaS deployment
5. SDK-first design with generated clients
6. Integration with existing SDKWork platform (Drive, IAM, ClawRouter)

## Decision

We will build the SDKWork Audio Application with the following architecture:

### 1. Domain Boundary
- **Domain**: `audio`
- **Capability**: `audio-workspace`
- **Package Type**: `rust-crate`

### 2. Crate Structure
```
sdkwork-audio/
├── crates/
│   ├── sdkwork-audio-core-rust/              # Domain models & business logic
│   ├── sdkwork-audio-storage-sqlx-rust/      # SQLite/PostgreSQL storage
│   ├── sdkwork-router-audio-app-api/         # App API route definitions
│   ├── sdkwork-router-audio-backend-api/     # Backend API route definitions
│   ├── sdkwork-audio-realtime-rust/          # Real-time processing
│   ├── sdkwork-audio-ai-engine-rust/         # AI engine integration
│   └── sdkwork-audio-drive-service/          # Drive integration
├── sdks/
│   ├── sdkwork-audio-app-sdk/                # App SDK family
│   └── sdkwork-audio-backend-sdk/            # Backend SDK family
└── packages/
    ├── sdkwork-audio-contracts/              # Shared contracts
    ├── sdkwork-audio-provider-adapter/       # Provider integration
    └── sdkwork-audio-pc-react/               # PC React UI
```

### 3. API Design
- **App API**: `/app/v3/api/audio/*` (45+ endpoints)
- **Backend API**: `/backend/v3/api/audio/*` (35+ endpoints)
- **OpenAPI Version**: 3.1.2
- **Security**: Dual-token (Authorization + Access-Token)

### 4. Database Design
- **50+ tables** covering complete audio domain
- **Multi-tenant**: All tables include `tenant_id`
- **Audit**: Full audit trail with timestamps
- **Soft Delete**: All tables support soft delete

### 5. SDK Design
- **Generator**: `@sdkwork/sdk-generator` / `sdkgen`
- **Profile**: `sdkwork-v3`
- **Output**: Resource-oriented client surface
- **Languages**: TypeScript (primary)

### 6. Integration Points
- **Drive Integration**: Audio asset storage via SDKWork Drive
- **ClawRouter**: AI provider routing and load balancing
- **AppBase IAM**: Authentication and authorization
- **MediaResource**: Media asset management

## Alternatives Considered

### Alternative 1: Extend sdkwork-voice
**Pros**: Reuse existing code, faster implementation
**Cons**: Voice is focused on speech, not comprehensive audio features
**Decision**: Rejected - Audio is a broader domain than voice

### Alternative 2: Build as separate microservices
**Pros**: Independent scaling, technology diversity
**Cons**: Increased complexity, harder to maintain consistency
**Decision**: Rejected - SDKWork prefers modular monolith architecture

### Alternative 3: Use existing audio SaaS platforms
**Pros**: Faster time to market, reduced development effort
**Cons**: Vendor lock-in, limited customization, higher costs
**Decision**: Rejected - Need full control over audio processing

## Consequences

### Benefits
1. **Comprehensive Platform**: All audio features in one platform
2. **Multi-engine Support**: Flexibility to use different AI providers
3. **Real-time Capabilities**: Live transcription and translation
4. **Professional Tools**: Audio workspace with advanced editing
5. **SDK-first Design**: Consistent client integration
6. **Multi-tenant Ready**: SaaS deployment capability

### Costs
1. **Development Effort**: Significant development required
2. **Complexity**: Multiple AI engines and real-time processing
3. **Maintenance**: Ongoing maintenance for multiple providers
4. **Infrastructure**: Real-time processing requires WebSocket infrastructure

## Verification

1. **Database Schema**: Verify all tables follow DATABASE_SPEC.md
2. **API Contracts**: Verify all endpoints follow API_SPEC.md
3. **SDK Generation**: Verify SDKs generate without warnings
4. **Integration Tests**: Verify integration with Drive, IAM, ClawRouter
5. **Performance Tests**: Verify latency and throughput targets
6. **Security Audit**: Verify security compliance

## Supersedes / Superseded By

This is a new architecture decision. No previous decision to supersede.

---

*Architecture decision recorded by SDKWork Audio Architecture System*
*Version: 1.0*
*Date: June 14, 2026*
