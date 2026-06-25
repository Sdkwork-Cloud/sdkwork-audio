# SDKWork Audio Application - Database Design Specification

## 1. Overview

This document defines the database schema for the SDKWork Audio Application. It follows the `DATABASE_SPEC.md` standards and implements a professional audio domain model.

## 2. Design Principles

### 2.1 Compliance Level
- **Target Level**: L2 (Service Ready)
- **Multi-tenant**: All tables include `tenant_id`
- **Audit**: Full audit trail with `created_at`, `updated_at`, `created_by`, `updated_by`
- **Soft Delete**: All tables support soft delete via `deleted_at`

### 2.2 Naming Conventions
- **Table Prefix**: `audio_`
- **Column Names**: Snake case
- **Index Names**: `idx_audio_<table>_<columns>`
- **Unique Names**: `uk_audio_<table>_<columns>`

### 2.3 Common Fields
All tables include:
```sql
id BIGINT PRIMARY KEY AUTOINCREMENT
uuid VARCHAR(64) NOT NULL UNIQUE
tenant_id BIGINT NOT NULL
organization_id BIGINT NOT NULL DEFAULT 0
created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
created_by BIGINT
updated_by BIGINT
deleted_at TIMESTAMP
```

## 3. Core Audio Domain Tables

### 3.1 audio_voice

Voice profiles for speech synthesis.

```sql
CREATE TABLE audio_voice (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    
    -- Voice metadata
    name VARCHAR(255) NOT NULL,
    description TEXT,
    language VARCHAR(10) NOT NULL,
    gender VARCHAR(20),
    age_group VARCHAR(20),
    accent VARCHAR(50),
    
    -- Voice characteristics
    pitch FLOAT DEFAULT 1.0,
    speed FLOAT DEFAULT 1.0,
    timbre VARCHAR(50),
    emotion_support BOOLEAN DEFAULT FALSE,
    
    -- Provider information
    provider_code VARCHAR(50) NOT NULL,
    provider_voice_id VARCHAR(255),
    provider_config JSONB,
    
    -- Voice type
    voice_type VARCHAR(50) NOT NULL DEFAULT 'prebuilt', -- prebuilt, custom, cloned
    clone_reference_url VARCHAR(1024),
    clone_parameters JSONB,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- active, inactive, deprecated
    is_public BOOLEAN DEFAULT FALSE,
    usage_count BIGINT DEFAULT 0,
    
    -- Quality metrics
    quality_score FLOAT,
    naturalness_score FLOAT,
    similarity_score FLOAT,
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by BIGINT,
    updated_by BIGINT,
    deleted_at TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_voice_tenant FOREIGN KEY (tenant_id) REFERENCES tenants(id),
    CONSTRAINT fk_audio_voice_organization FOREIGN KEY (organization_id) REFERENCES organizations(id),
    CONSTRAINT chk_audio_voice_status CHECK (status IN ('active', 'inactive', 'deprecated')),
    CONSTRAINT chk_audio_voice_type CHECK (voice_type IN ('prebuilt', 'custom', 'cloned'))
);

-- Indexes
CREATE INDEX idx_audio_voice_tenant_status ON audio_voice(tenant_id, status);
CREATE INDEX idx_audio_voice_language ON audio_voice(language);
CREATE INDEX idx_audio_voice_provider ON audio_voice(provider_code);
CREATE INDEX idx_audio_voice_type ON audio_voice(voice_type);
```

### 3.2 audio_generation_task

Core task table for all audio generation operations.

```sql
CREATE TABLE audio_generation_task (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    user_id BIGINT NOT NULL,
    
    -- Task metadata
    operation_type VARCHAR(50) NOT NULL, -- speech, transcription, translation, sound_effect, music
    title VARCHAR(255),
    description TEXT,
    
    -- Provider routing
    provider_code VARCHAR(50),
    provider_route_id BIGINT,
    provider_task_id VARCHAR(255),
    provider_request_id VARCHAR(255),
    
    -- Status tracking
    status VARCHAR(50) NOT NULL DEFAULT 'queued', -- queued, routing, submitted, running, succeeded, failed, cancelled, expired, needs_review
    status_message TEXT,
    progress FLOAT DEFAULT 0.0,
    
    -- Request parameters
    request_params JSONB NOT NULL,
    
    -- Result metadata
    result_metadata JSONB,
    duration_ms BIGINT,
    token_count BIGINT,
    character_count BIGINT,
    
    -- Idempotency
    idempotency_key VARCHAR(255),
    
    -- Priority and scheduling
    priority INTEGER DEFAULT 0,
    scheduled_at TIMESTAMP,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    
    -- Error handling
    error_code VARCHAR(50),
    error_message TEXT,
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by BIGINT,
    updated_by BIGINT,
    deleted_at TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_generation_task_tenant FOREIGN KEY (tenant_id) REFERENCES tenants(id),
    CONSTRAINT fk_audio_generation_task_user FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT chk_audio_generation_task_operation CHECK (operation_type IN ('speech', 'transcription', 'translation', 'sound_effect', 'music')),
    CONSTRAINT chk_audio_generation_task_status CHECK (status IN ('queued', 'routing', 'submitted', 'running', 'succeeded', 'failed', 'cancelled', 'expired', 'needs_review')),
    CONSTRAINT uk_audio_generation_task_idempotency UNIQUE (tenant_id, user_id, idempotency_key)
);

-- Indexes
CREATE INDEX idx_audio_generation_task_tenant_user ON audio_generation_task(tenant_id, user_id);
CREATE INDEX idx_audio_generation_task_status ON audio_generation_task(tenant_id, status);
CREATE INDEX idx_audio_generation_task_operation ON audio_generation_task(operation_type);
CREATE INDEX idx_audio_generation_task_provider ON audio_generation_task(provider_code, provider_task_id);
CREATE INDEX idx_audio_generation_task_created ON audio_generation_task(tenant_id, created_at DESC);
```

### 3.3 audio_task_event

Event sourcing for task lifecycle.

```sql
CREATE TABLE audio_task_event (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    task_id BIGINT NOT NULL,
    
    -- Event metadata
    event_type VARCHAR(50) NOT NULL, -- status_change, progress_update, error, retry, webhook
    event_data JSONB NOT NULL,
    
    -- Status transition
    from_status VARCHAR(50),
    to_status VARCHAR(50),
    
    -- Provider information
    provider_code VARCHAR(50),
    provider_event_id VARCHAR(255),
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by BIGINT,
    
    -- Constraints
    CONSTRAINT fk_audio_task_event_task FOREIGN KEY (task_id) REFERENCES audio_generation_task(id),
    CONSTRAINT chk_audio_task_event_type CHECK (event_type IN ('status_change', 'progress_update', 'error', 'retry', 'webhook', 'cancel', 'reconcile'))
);

-- Indexes
CREATE INDEX idx_audio_task_event_task ON audio_task_event(task_id, created_at DESC);
CREATE INDEX idx_audio_task_event_type ON audio_task_event(event_type);
```

### 3.4 audio_artifact

Generated audio artifacts.

```sql
CREATE TABLE audio_artifact (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    task_id BIGINT NOT NULL,
    
    -- Artifact metadata
    artifact_index INTEGER NOT NULL DEFAULT 0,
    artifact_kind VARCHAR(50) NOT NULL, -- audio, text, subtitle, waveform
    media_kind VARCHAR(50) NOT NULL, -- speech, transcription, translation, sound_effect
    
    -- Content information
    mime_type VARCHAR(100),
    content_length BIGINT,
    duration_ms BIGINT,
    sample_rate INTEGER,
    channels INTEGER,
    bit_rate INTEGER,
    
    -- Storage
    source_type VARCHAR(50) NOT NULL DEFAULT 'provider', -- provider, drive, url
    source_url VARCHAR(1024),
    drive_space_id BIGINT,
    drive_node_id BIGINT,
    checksum VARCHAR(64),
    
    -- Provider information
    provider_asset_id VARCHAR(255),
    provider_metadata JSONB,
    
    -- Content
    text_content TEXT,
    subtitle_content JSONB,
    waveform_data JSONB,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, processing, ready, failed
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_artifact_task FOREIGN KEY (task_id) REFERENCES audio_generation_task(id),
    CONSTRAINT chk_audio_artifact_kind CHECK (artifact_kind IN ('audio', 'text', 'subtitle', 'waveform', 'metadata')),
    CONSTRAINT chk_audio_artifact_media CHECK (media_kind IN ('speech', 'transcription', 'translation', 'sound_effect', 'music')),
    CONSTRAINT chk_audio_artifact_status CHECK (status IN ('pending', 'processing', 'ready', 'failed'))
);

-- Indexes
CREATE INDEX idx_audio_artifact_task ON audio_artifact(task_id, artifact_index);
CREATE INDEX idx_audio_artifact_tenant ON audio_artifact(tenant_id);
CREATE INDEX idx_audio_artifact_status ON audio_artifact(status);
CREATE INDEX idx_audio_artifact_media ON audio_artifact(media_kind);
```

### 3.5 audio_artifact_drive_sync

Drive synchronization for artifacts.

```sql
CREATE TABLE audio_artifact_drive_sync (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    artifact_id BIGINT NOT NULL,
    
    -- Sync status
    status VARCHAR(50) NOT NULL DEFAULT 'pending_upload', -- pending_upload, uploading, uploaded, failed, skipped, deleted
    
    -- Drive information
    drive_space_id BIGINT,
    drive_node_id BIGINT,
    drive_resource_uri VARCHAR(1024),
    
    -- Upload details
    upload_item_id BIGINT,
    upload_session_id BIGINT,
    
    -- Error handling
    error_code VARCHAR(50),
    error_message TEXT,
    retry_count INTEGER DEFAULT 0,
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_artifact_drive_sync_artifact FOREIGN KEY (artifact_id) REFERENCES audio_artifact(id),
    CONSTRAINT chk_audio_artifact_drive_sync_status CHECK (status IN ('pending_upload', 'uploading', 'uploaded', 'failed', 'skipped', 'deleted'))
);

-- Indexes
CREATE INDEX idx_audio_artifact_drive_sync_artifact ON audio_artifact_drive_sync(artifact_id);
CREATE INDEX idx_audio_artifact_drive_sync_status ON audio_artifact_drive_sync(status);
```

## 4. Speech Synthesis Tables

### 4.1 audio_speech_request

```sql
CREATE TABLE audio_speech_request (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    task_id BIGINT NOT NULL,
    
    -- Input
    text TEXT NOT NULL,
    text_format VARCHAR(20) DEFAULT 'plain', -- plain, ssml
    language VARCHAR(10),
    
    -- Voice settings
    voice_id BIGINT,
    voice_name VARCHAR(255),
    
    -- Audio settings
    speed FLOAT DEFAULT 1.0,
    pitch FLOAT DEFAULT 1.0,
    volume FLOAT DEFAULT 1.0,
    sample_rate INTEGER DEFAULT 24000,
    audio_format VARCHAR(20) DEFAULT 'mp3', -- mp3, wav, flac, ogg
    
    -- Emotion and style
    emotion VARCHAR(50),
    emotion_intensity FLOAT DEFAULT 0.5,
    style VARCHAR(50),
    style_intensity FLOAT DEFAULT 0.5,
    
    -- Advanced
    pronunciation_dict JSONB,
    ssml_options JSONB,
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_speech_request_task FOREIGN KEY (task_id) REFERENCES audio_generation_task(id),
    CONSTRAINT fk_audio_speech_request_voice FOREIGN KEY (voice_id) REFERENCES audio_voice(id)
);

-- Indexes
CREATE INDEX idx_audio_speech_request_task ON audio_speech_request(task_id);
CREATE INDEX idx_audio_speech_request_voice ON audio_speech_request(voice_id);
```

## 5. Transcription Tables

### 5.1 audio_transcription_request

```sql
CREATE TABLE audio_transcription_request (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    task_id BIGINT NOT NULL,
    
    -- Input
    audio_url VARCHAR(1024),
    audio_format VARCHAR(20),
    audio_duration_ms BIGINT,
    
    -- Settings
    language VARCHAR(10),
    detect_language BOOLEAN DEFAULT TRUE,
    enable_timestamps BOOLEAN DEFAULT TRUE,
    enable_speaker_diarization BOOLEAN DEFAULT FALSE,
    max_speakers INTEGER,
    
    -- Vocabulary
    vocabulary_id BIGINT,
    custom_vocabulary JSONB,
    
    -- Output options
    output_format VARCHAR(20) DEFAULT 'json', -- json, srt, vtt, txt
    include_confidence BOOLEAN DEFAULT TRUE,
    include_words BOOLEAN DEFAULT TRUE,
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_transcription_request_task FOREIGN KEY (task_id) REFERENCES audio_generation_task(id)
);

-- Indexes
CREATE INDEX idx_audio_transcription_request_task ON audio_transcription_request(task_id);
```

### 5.2 audio_transcription_segment

```sql
CREATE TABLE audio_transcription_segment (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    task_id BIGINT NOT NULL,
    
    -- Segment timing
    start_ms BIGINT NOT NULL,
    end_ms BIGINT NOT NULL,
    
    -- Content
    text TEXT NOT NULL,
    language VARCHAR(10),
    
    -- Speaker
    speaker_id INTEGER,
    speaker_label VARCHAR(50),
    
    -- Confidence
    confidence FLOAT,
    
    -- Words
    words JSONB,
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_transcription_segment_task FOREIGN KEY (task_id) REFERENCES audio_generation_task(id)
);

-- Indexes
CREATE INDEX idx_audio_transcription_segment_task ON audio_transcription_segment(task_id, start_ms);
CREATE INDEX idx_audio_transcription_segment_speaker ON audio_transcription_segment(task_id, speaker_id);
```

## 6. Real-time Tables

### 6.1 audio_realtime_session

```sql
CREATE TABLE audio_realtime_session (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    
    -- Session metadata
    session_type VARCHAR(50) NOT NULL, -- transcription, translation, call
    title VARCHAR(255),
    
    -- Configuration
    language VARCHAR(10),
    target_language VARCHAR(10),
    enable_translation BOOLEAN DEFAULT FALSE,
    enable_speaker_diarization BOOLEAN DEFAULT FALSE,
    
    -- Connection
    connection_id VARCHAR(255),
    websocket_url VARCHAR(1024),
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'created', -- created, connecting, active, paused, ended, failed
    
    -- Participants
    participant_count INTEGER DEFAULT 0,
    max_participants INTEGER DEFAULT 10,
    
    -- Timing
    started_at TIMESTAMP,
    ended_at TIMESTAMP,
    duration_ms BIGINT,
    
    -- Recording
    is_recording BOOLEAN DEFAULT FALSE,
    recording_url VARCHAR(1024),
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_realtime_session_user FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT chk_audio_realtime_session_type CHECK (session_type IN ('transcription', 'translation', 'call')),
    CONSTRAINT chk_audio_realtime_session_status CHECK (status IN ('created', 'connecting', 'active', 'paused', 'ended', 'failed'))
);

-- Indexes
CREATE INDEX idx_audio_realtime_session_user ON audio_realtime_session(user_id, status);
CREATE INDEX idx_audio_realtime_session_tenant ON audio_realtime_session(tenant_id, created_at DESC);
```

## 7. Provider Tables

### 7.1 audio_provider_route

```sql
CREATE TABLE audio_provider_route (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    
    -- Provider information
    provider_code VARCHAR(50) NOT NULL,
    provider_name VARCHAR(255) NOT NULL,
    provider_type VARCHAR(50) NOT NULL, -- openai, elevenlabs, azure, google, custom
    
    -- Configuration
    base_url VARCHAR(1024),
    api_key_encrypted TEXT,
    config JSONB,
    
    -- Capabilities
    capabilities JSONB NOT NULL, -- ["speech", "transcription", "translation"]
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- active, inactive, maintenance
    priority INTEGER DEFAULT 0,
    
    -- Rate limiting
    rate_limit_per_minute INTEGER,
    rate_limit_per_day INTEGER,
    
    -- Health
    health_status VARCHAR(50) DEFAULT 'healthy', -- healthy, degraded, unhealthy
    last_health_check_at TIMESTAMP,
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by BIGINT,
    updated_by BIGINT,
    deleted_at TIMESTAMP,
    
    -- Constraints
    CONSTRAINT uk_audio_provider_route_code UNIQUE (tenant_id, provider_code),
    CONSTRAINT chk_audio_provider_route_status CHECK (status IN ('active', 'inactive', 'maintenance')),
    CONSTRAINT chk_audio_provider_route_health CHECK (health_status IN ('healthy', 'degraded', 'unhealthy'))
);

-- Indexes
CREATE INDEX idx_audio_provider_route_tenant ON audio_provider_route(tenant_id, status);
CREATE INDEX idx_audio_provider_route_type ON audio_provider_route(provider_type);
```

## 8. Workspace Tables

### 8.1 audio_workspace

```sql
CREATE TABLE audio_workspace (
    id BIGINT PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    
    -- Workspace metadata
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Settings
    settings JSONB,
    
    -- Collaboration
    is_public BOOLEAN DEFAULT FALSE,
    collaborator_count INTEGER DEFAULT 0,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- active, archived, deleted
    
    -- Audit
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    
    -- Constraints
    CONSTRAINT fk_audio_workspace_user FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT chk_audio_workspace_status CHECK (status IN ('active', 'archived', 'deleted'))
);

-- Indexes
CREATE INDEX idx_audio_workspace_user ON audio_workspace(user_id, status);
CREATE INDEX idx_audio_workspace_tenant ON audio_workspace(tenant_id, created_at DESC);
```

## 9. Migration Strategy

### 9.1 Migration Files

```
migrations/
├── 0001_audio_core.sql           -- Core tables
├── 0002_audio_speech.sql         -- Speech synthesis tables
├── 0003_audio_transcription.sql  -- Transcription tables
├── 0004_audio_translation.sql    -- Translation tables
├── 0005_audio_sound_effect.sql   -- Sound effect tables
├── 0006_audio_realtime.sql       -- Real-time tables
├── 0007_audio_workspace.sql      -- Workspace tables
└── 0008_audio_provider.sql       -- Provider tables
```

### 9.2 Migration Rules

1. **Forward-only**: No rollback migrations
2. **Idempotent**: Safe to run multiple times
3. **Ordered**: Sequential numbering
4. **Documented**: Each migration has description
5. **Tested**: Migration tests required

## 10. Index Strategy Summary

| Table | Index Type | Columns | Purpose |
|-------|-----------|---------|---------|
| audio_voice | Composite | (tenant_id, status) | Tenant voice listing |
| audio_voice | B-tree | language | Language filtering |
| audio_generation_task | Composite | (tenant_id, user_id) | User task listing |
| audio_generation_task | Composite | (tenant_id, status) | Status filtering |
| audio_generation_task | Composite | (provider_code, provider_task_id) | Provider lookup |
| audio_artifact | Composite | (task_id, artifact_index) | Task artifacts |
| audio_transcription_segment | Composite | (task_id, start_ms) | Segment ordering |
| audio_realtime_session | Composite | (user_id, status) | User sessions |

---

*Document generated by SDKWork Audio Database Design System*
*Version: 1.0*
*Date: June 14, 2026*
