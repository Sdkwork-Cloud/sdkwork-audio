> Migrated from `docs/API_DESIGN.md` on 2026-06-24.
> Owner: SDKWork maintainers

## 1. Overview

This document defines the HTTP API contracts for the SDKWork Audio Application. It follows the `API_SPEC.md` standards and implements OpenAPI 3.1.2 stable profile.

## 2. API Design Principles

### 2.1 Compliance Level
- **Target Level**: L2 (Service Ready)
- **OpenAPI Version**: 3.1.2
- **Security**: Dual-token (Authorization + Access-Token)
- **Versioning**: URL-based (`/v3/api/`)

### 2.2 Naming Conventions
- **Operation IDs**: Dot-separated (e.g., `speech.create`)
- **Paths**: Kebab-case (e.g., `/sound-effects`)
- **Schemas**: PascalCase (e.g., `SpeechRequest`)
- **Parameters**: camelCase (e.g., `voiceId`)

### 2.3 Standard Response Envelope

```json
{
  "code": 0,
  "message": "success",
  "data": {},
  "metadata": {
    "requestId": "uuid",
    "timestamp": "2026-06-14T10:00:00Z"
  }
}
```

### 2.4 Error Response (RFC 7807)

```json
{
  "type": "https://sdkwork.com/errors/validation-error",
  "title": "Validation Error",
  "status": 400,
  "detail": "The request body is invalid",
  "instance": "/app/v3/api/audio/speech",
  "errors": [
    {
      "field": "text",
      "message": "Text is required"
    }
  ]
}
```

## 3. API Surfaces

### 3.1 App API (`/app/v3/api/audio/*`)

**Target**: Mobile and web applications
**Authentication**: Dual-token required
**Rate Limit**: 100 requests/minute per user

### 3.2 Backend API (`/backend/v3/api/audio/*`)

**Target**: Administration and management
**Authentication**: Dual-token required
**Rate Limit**: 1000 requests/minute per tenant

## 4. App API Operations

### 4.1 Speech Synthesis

#### `speech.create`

Create a speech synthesis task.

**Operation ID**: `speech.create`
**Method**: `POST`
**Path**: `/app/v3/api/audio/speech`
**Auth**: Required

**Request Body**:
```json
{
  "text": "Hello, this is a test.",
  "textFormat": "plain",
  "language": "en",
  "voiceId": "voice-uuid",
  "speed": 1.0,
  "pitch": 1.0,
  "volume": 1.0,
  "emotion": "neutral",
  "emotionIntensity": 0.5,
  "audioFormat": "mp3",
  "sampleRate": 24000,
  "idempotencyKey": "unique-key"
}
```

**Response**:
```json
{
  "code": 0,
  "data": {
    "taskId": "task-uuid",
    "status": "queued",
    "estimatedDuration": 2000
  }
}
```

#### `speech.voices.list`

List available voices.

**Operation ID**: `speech.voices.list`
**Method**: `GET`
**Path**: `/app/v3/api/audio/speech/voices`
**Auth**: Required

**Query Parameters**:
- `language` (string): Filter by language
- `gender` (string): Filter by gender
- `voiceType` (string): Filter by type (prebuilt, custom, cloned)
- `cursor` (string): Pagination cursor
- `limit` (integer, max 100): Number of results

**Response**:
```json
{
  "code": 0,
  "data": {
    "items": [
      {
        "voiceId": "voice-uuid",
        "name": "English Male",
        "language": "en",
        "gender": "male",
        "voiceType": "prebuilt",
        "previewUrl": "https://..."
      }
    ],
    "nextCursor": "cursor-string",
    "hasMore": true
  }
}
```

### 4.2 Transcription

#### `transcriptions.create`

Create a transcription task.

**Operation ID**: `transcriptions.create`
**Method**: `POST`
**Path**: `/app/v3/api/audio/transcriptions`
**Auth**: Required

**Request Body**:
```json
{
  "audioUrl": "https://example.com/audio.mp3",
  "language": "en",
  "detectLanguage": true,
  "enableTimestamps": true,
  "enableSpeakerDiarization": true,
  "maxSpeakers": 5,
  "outputFormat": "json",
  "includeConfidence": true,
  "includeWords": true,
  "idempotencyKey": "unique-key"
}
```

**Response**:
```json
{
  "code": 0,
  "data": {
    "taskId": "task-uuid",
    "status": "queued"
  }
}
```

#### `transcriptions.segments.list`

List transcription segments.

**Operation ID**: `transcriptions.segments.list`
**Method**: `GET`
**Path**: `/app/v3/api/audio/transcriptions/{taskId}/segments`
**Auth**: Required

**Response**:
```json
{
  "code": 0,
  "data": {
    "items": [
      {
        "segmentId": "segment-uuid",
        "startMs": 0,
        "endMs": 5000,
        "text": "Hello, this is a test.",
        "speakerId": 1,
        "speakerLabel": "Speaker 1",
        "confidence": 0.95,
        "words": [
          {
            "word": "Hello",
            "startMs": 0,
            "endMs": 500,
            "confidence": 0.98
          }
        ]
      }
    ]
  }
}
```

### 4.3 Translation

#### `translations.create`

Create a translation task.

**Operation ID**: `translations.create`
**Method**: `POST`
**Path**: `/app/v3/api/audio/translations`
**Auth**: Required

**Request Body**:
```json
{
  "audioUrl": "https://example.com/audio.mp3",
  "sourceLanguage": "en",
  "targetLanguage": "zh",
  "outputFormat": "json",
  "includeTimestamps": true,
  "idempotencyKey": "unique-key"
}
```

**Response**:
```json
{
  "code": 0,
  "data": {
    "taskId": "task-uuid",
    "status": "queued"
  }
}
```

### 4.4 Sound Effects

#### `soundEffects.create`

Generate a sound effect.

**Operation ID**: `soundEffects.create`
**Method**: `POST`
**Path**: `/app/v3/api/audio/sound-effects`
**Auth**: Required

**Request Body**:
```json
{
  "description": "Rain falling on a window",
  "duration": 5000,
  "style": "realistic",
  "intensity": 0.7,
  "format": "wav",
  "sampleRate": 44100,
  "idempotencyKey": "unique-key"
}
```

**Response**:
```json
{
  "code": 0,
  "data": {
    "taskId": "task-uuid",
    "status": "queued"
  }
}
```

### 4.5 Task Management

#### `tasks.list`

List user's tasks.

**Operation ID**: `tasks.list`
**Method**: `GET`
**Path**: `/app/v3/api/audio/tasks`
**Auth**: Required

**Query Parameters**:
- `operationType` (string): Filter by operation type
- `status` (string): Filter by status
- `cursor` (string): Pagination cursor
- `limit` (integer, max 100): Number of results

**Response**:
```json
{
  "code": 0,
  "data": {
    "items": [
      {
        "taskId": "task-uuid",
        "operationType": "speech",
        "status": "succeeded",
        "createdAt": "2026-06-14T10:00:00Z",
        "completedAt": "2026-06-14T10:00:02Z"
      }
    ],
    "nextCursor": "cursor-string",
    "hasMore": true
  }
}
```

#### `tasks.retrieve`

Get task details.

**Operation ID**: `tasks.retrieve`
**Method**: `GET`
**Path**: `/app/v3/api/audio/tasks/{taskId}`
**Auth**: Required

**Response**:
```json
{
  "code": 0,
  "data": {
    "taskId": "task-uuid",
    "operationType": "speech",
    "status": "succeeded",
    "progress": 1.0,
    "requestParams": {},
    "resultMetadata": {},
    "artifacts": [
      {
        "artifactId": "artifact-uuid",
        "artifactKind": "audio",
        "mediaKind": "speech",
        "mimeType": "audio/mpeg",
        "contentLength": 12345,
        "durationMs": 2000,
        "downloadUrl": "https://..."
      }
    ],
    "createdAt": "2026-06-14T10:00:00Z",
    "completedAt": "2026-06-14T10:00:02Z"
  }
}
```

#### `tasks.cancel`

Cancel a task.

**Operation ID**: `tasks.cancel`
**Method**: `POST`
**Path**: `/app/v3/api/audio/tasks/{taskId}/cancel`
**Auth**: Required

**Response**:
```json
{
  "code": 0,
  "data": {
    "taskId": "task-uuid",
    "status": "cancelled"
  }
}
```

### 4.6 Real-time Sessions

#### `realtime.sessions.create`

Create a real-time session.

**Operation ID**: `realtime.sessions.create`
**Method**: `POST`
**Path**: `/app/v3/api/audio/realtime/sessions`
**Auth**: Required

**Request Body**:
```json
{
  "sessionType": "transcription",
  "language": "en",
  "enableTranslation": true,
  "targetLanguage": "zh",
  "enableSpeakerDiarization": true,
  "maxSpeakers": 5,
  "isRecording": true
}
```

**Response**:
```json
{
  "code": 0,
  "data": {
    "sessionId": "session-uuid",
    "websocketUrl": "wss://...",
    "status": "created"
  }
}
```

### 4.7 Voice Management

#### `voices.list`

List user's custom voices.

**Operation ID**: `voices.list`
**Method**: `GET`
**Path**: `/app/v3/api/audio/voices`
**Auth**: Required

#### `voices.create`

Create a custom voice.

**Operation ID**: `voices.create`
**Method**: `POST`
**Path**: `/app/v3/api/audio/voices`
**Auth**: Required

**Request Body**:
```json
{
  "name": "My Custom Voice",
  "description": "A custom voice for my project",
  "language": "en",
  "referenceAudioUrl": "https://...",
  "cloneParameters": {
    "quality": "high",
    "stability": 0.5
  }
}
```

### 4.8 Workspace

#### `workspaces.list`

List user's workspaces.

**Operation ID**: `workspaces.list`
**Method**: `GET`
**Path**: `/app/v3/api/audio/workspaces`
**Auth**: Required

#### `workspaces.create`

Create a workspace.

**Operation ID**: `workspaces.create`
**Method**: `POST`
**Path**: `/app/v3/api/audio/workspaces`
**Auth**: Required

**Request Body**:
```json
{
  "name": "My Audio Project",
  "description": "A workspace for my audio project"
}
```

### 4.9 Export

#### `exports.create`

Create an export task.

**Operation ID**: `exports.create`
**Method**: `POST`
**Path**: `/app/v3/api/audio/exports`
**Auth**: Required

**Request Body**:
```json
{
  "workspaceId": "workspace-uuid",
  "format": "mp3",
  "quality": "high",
  "sampleRate": 44100,
  "channels": 2
}
```

## 5. Backend API Operations

### 5.1 Provider Management

#### `providerRoutes.create`

Create a provider route.

**Operation ID**: `providerRoutes.create`
**Method**: `POST`
**Path**: `/backend/v3/api/audio/provider-routes`
**Auth**: Required

**Request Body**:
```json
{
  "providerCode": "openai",
  "providerName": "OpenAI",
  "providerType": "openai",
  "baseUrl": "https://api.openai.com/v1",
  "apiKey": "sk-...",
  "capabilities": ["speech", "transcription", "translation"],
  "config": {
    "model": "tts-1",
    "voice": "alloy"
  }
}
```

#### `providerRoutes.list`

List provider routes.

**Operation ID**: `providerRoutes.list`
**Method**: `GET`
**Path**: `/backend/v3/api/audio/provider-routes`
**Auth**: Required

#### `providerRoutes.update`

Update a provider route.

**Operation ID**: `providerRoutes.update`
**Method**: `PATCH`
**Path**: `/backend/v3/api/audio/provider-routes/{providerRouteId}`
**Auth**: Required

### 5.2 Task Administration

#### `tasks.list`

List all tasks (admin).

**Operation ID**: `tasks.list`
**Method**: `GET`
**Path**: `/backend/v3/api/audio/tasks`
**Auth**: Required

**Query Parameters**:
- `userId` (string): Filter by user
- `operationType` (string): Filter by operation type
- `status` (string): Filter by status
- `cursor` (string): Pagination cursor
- `limit` (integer, max 100): Number of results

#### `tasks.retry`

Retry a failed task.

**Operation ID**: `tasks.retry`
**Method**: `POST`
**Path**: `/backend/v3/api/audio/tasks/{taskId}/retry`
**Auth**: Required

#### `tasks.reconcile`

Reconcile a stuck task.

**Operation ID**: `tasks.reconcile`
**Method**: `POST`
**Path**: `/backend/v3/api/audio/tasks/{taskId}/reconcile`
**Auth**: Required

### 5.3 Webhook Management

#### `providerWebhooks.accept`

Accept a provider webhook.

**Operation ID**: `providerWebhooks.accept`
**Method**: `POST`
**Path**: `/backend/v3/api/audio/provider-webhooks/{providerCode}`
**Auth**: API Key

#### `webhookEvents.list`

List webhook events.

**Operation ID**: `webhookEvents.list`
**Method**: `GET`
**Path**: `/backend/v3/api/audio/webhook-events`
**Auth**: Required

#### `webhookEvents.replay`

Replay a webhook event.

**Operation ID**: `webhookEvents.replay`
**Method**: `POST`
**Path**: `/backend/v3/api/audio/webhook-events/{eventId}/replay`
**Auth**: Required

### 5.4 Analytics

#### `analytics.usage`

Get usage analytics.

**Operation ID**: `analytics.usage`
**Method**: `GET`
**Path**: `/backend/v3/api/audio/analytics/usage`
**Auth**: Required

**Query Parameters**:
- `startDate` (string): Start date
- `endDate` (string): End date
- `groupBy` (string): Group by (day, week, month)
- `operationType` (string): Filter by operation type

**Response**:
```json
{
  "code": 0,
  "data": {
    "totalRequests": 1000,
    "totalDuration": 500000,
    "totalCharacters": 1000000,
    "breakdown": [
      {
        "date": "2026-06-14",
        "requests": 100,
        "duration": 50000,
        "characters": 100000
      }
    ]
  }
}
```

## 6. OpenAPI Specification Structure

### 6.1 File Organization

```
apis/
├── open-api/
│   └── audio/
│       └── openapi.yaml
├── app-api/
│   └── audio/
│       ├── openapi.yaml
│       ├── routes/
│       │   ├── speech.yaml
│       │   ├── transcriptions.yaml
│       │   ├── translations.yaml
│       │   ├── sound-effects.yaml
│       │   ├── tasks.yaml
│       │   ├── realtime.yaml
│       │   ├── voices.yaml
│       │   ├── workspaces.yaml
│       │   └── exports.yaml
│       ├── schemas/
│       │   ├── SpeechRequest.yaml
│       │   ├── TranscriptionRequest.yaml
│       │   ├── TaskResponse.yaml
│       │   └── ...
│       ├── examples/
│       └── changelogs/
└── backend-api/
    └── audio/
        ├── openapi.yaml
        ├── routes/
        ├── schemas/
        ├── examples/
        └── changelogs/
```

### 6.2 Security Schemes

```yaml
components:
  securitySchemes:
    AuthToken:
      type: apiKey
      in: header
      name: Authorization
      description: JWT access token
    AccessToken:
      type: apiKey
      in: header
      name: Access-Token
      description: Access token for API access

security:
  - AuthToken: []
  - AccessToken: []
```

### 6.3 Common Response Schemas

```yaml
components:
  schemas:
    SuccessResponse:
      type: object
      properties:
        code:
          type: integer
          example: 0
        message:
          type: string
          example: success
        data:
          type: object
        metadata:
          $ref: '#/components/schemas/ResponseMetadata'
    
    ErrorResponse:
      type: object
      properties:
        type:
          type: string
          example: https://sdkwork.com/errors/validation-error
        title:
          type: string
          example: Validation Error
        status:
          type: integer
          example: 400
        detail:
          type: string
        instance:
          type: string
        errors:
          type: array
          items:
            $ref: '#/components/schemas/ErrorDetail'
    
    ResponseMetadata:
      type: object
      properties:
        requestId:
          type: string
          format: uuid
        timestamp:
          type: string
          format: date-time
```

## 7. Rate Limiting

### 7.1 Rate Limit Headers

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 99
X-RateLimit-Reset: 1623456789
```

### 7.2 Rate Limit Tiers

| Tier | Requests/Minute | Requests/Day |
|------|----------------|--------------|
| Free | 10 | 100 |
| Basic | 100 | 10,000 |
| Pro | 1,000 | 100,000 |
| Enterprise | 10,000 | 1,000,000 |

## 8. Pagination

### 8.1 Cursor-based Pagination

**Request**:
```
GET /app/v3/api/audio/tasks?cursor=abc123&limit=20
```

**Response**:
```json
{
  "data": {
    "items": [...],
    "nextCursor": "def456",
    "hasMore": true
  }
}
```

## 9. Idempotency

### 9.1 Idempotency Key

All create operations support idempotency keys.

**Header**:
```
X-Idempotency-Key: unique-key-123
```

**Rules**:
- Keys must be unique per user per operation
- Keys expire after 24 hours
- Duplicate requests return original response

## 10. Webhook Events

### 10.1 Webhook Payload

```json
{
  "eventId": "event-uuid",
  "eventType": "task.completed",
  "timestamp": "2026-06-14T10:00:00Z",
  "data": {
    "taskId": "task-uuid",
    "status": "succeeded",
    "artifacts": [...]
  }
}
```

### 10.2 Event Types

| Event | Description |
|-------|-------------|
| `task.created` | Task created |
| `task.started` | Task started processing |
| `task.progress` | Task progress update |
| `task.completed` | Task completed successfully |
| `task.failed` | Task failed |
| `task.cancelled` | Task cancelled |

---

*Document generated by SDKWork Audio API Design System*
*Version: 1.0*
*Date: June 14, 2026*

