# SDKWork Audio Application - SDK Design Specification

## 1. Overview

This document defines the SDK architecture for the SDKWork Audio Application. It follows the `SDK_SPEC.md` standards and implements generated TypeScript clients.

## 2. SDK Design Principles

### 2.1 Compliance Level
- **Generator**: `@sdkwork/sdk-generator` / `sdkgen`
- **Profile**: `sdkwork-v3`
- **Output**: Resource-oriented client surface
- **Language**: TypeScript (primary), extensible to other languages

### 2.2 Naming Conventions
- **SDK Family**: `sdkwork-audio-app-sdk`, `sdkwork-audio-backend-sdk`
- **Package Name**: `@sdkwork/audio-app-sdk`, `@sdkwork/audio-backend-sdk`
- **Client Class**: `AudioAppClient`, `AudioBackendClient`
- **Methods**: Resource-oriented (e.g., `client.speech.create()`)

### 2.3 SDK Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SDK Family Structure                       │
│                                                              │
│  ┌─────────────────────┐    ┌─────────────────────┐        │
│  │ sdkwork-audio-app-sdk│    │sdkwork-audio-backend-sdk│     │
│  │  - TypeScript         │    │  - TypeScript         │        │
│  │  - App API surface    │    │  - Backend API surface│        │
│  └──────────┬──────────┘    └──────────┬──────────┘        │
│             │                          │                    │
│  ┌──────────┴──────────────────────────┴──────────┐        │
│  │              Generated Transport Layer          │        │
│  │  - HTTP client with auth injection              │        │
│  │  - Request/response serialization               │        │
│  │  - Error handling                               │        │
│  │  - WebSocket support for real-time              │        │
│  └─────────────────────────────────────────────────┘        │
│                                                              │
│  ┌─────────────────────────────────────────────────┐        │
│  │              SDK Dependencies                    │        │
│  │  - clawrouter-open-sdk (AI generation)          │        │
│  │  - drive-sdk (file storage)                     │        │
│  │  - iam-sdk (authentication)                     │        │
│  │  - media-resource-sdk (media management)        │        │
│  └─────────────────────────────────────────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## 3. SDK Families

### 3.1 App SDK

**Package**: `@sdkwork/audio-app-sdk`
**Authority**: `sdkwork-audio-app-api`
**Prefix**: `/app/v3/api/audio`

**Operations**:

| Category | Methods | Description |
|----------|---------|-------------|
| Speech | `speech.create()`, `speech.voices.list()`, `speech.voices.create()` | Speech synthesis |
| Transcription | `transcriptions.create()`, `transcriptions.segments.list()` | Audio transcription |
| Translation | `translations.create()`, `translations.languages.list()` | Audio translation |
| Sound Effects | `soundEffects.create()`, `soundEffects.presets.list()` | Sound effect generation |
| Tasks | `tasks.list()`, `tasks.retrieve()`, `tasks.cancel()` | Task management |
| Real-time | `realtime.sessions.create()`, `realtime.transcription.create()` | Real-time processing |
| Voices | `voices.list()`, `voices.create()`, `voices.clone()` | Voice management |
| Workspaces | `workspaces.list()`, `workspaces.create()`, `workspaces.tracks.list()` | Workspace management |
| Exports | `exports.create()`, `exports.formats.list()` | Export management |

### 3.2 Backend SDK

**Package**: `@sdkwork/audio-backend-sdk`
**Authority**: `sdkwork-audio-backend-api`
**Prefix**: `/backend/v3/api/audio`

**Operations**:

| Category | Methods | Description |
|----------|---------|-------------|
| Provider Routes | `providerRoutes.create()`, `providerRoutes.list()`, `providerRoutes.update()` | Provider management |
| Tasks | `tasks.list()`, `tasks.retrieve()`, `tasks.retry()`, `tasks.reconcile()` | Task administration |
| Webhooks | `providerWebhooks.accept()`, `webhookEvents.list()`, `webhookEvents.replay()` | Webhook management |
| Analytics | `analytics.usage()`, `analytics.quality()`, `analytics.performance()` | Usage analytics |
| Moderation | `moderation.flags.list()`, `moderation.flags.resolve()` | Content moderation |
| System | `system.health()`, `system.metrics()`, `system.config()` | System management |

## 4. SDK Generation

### 4.1 Generation Command

```powershell
# Generate App SDK
powershell -NoProfile -ExecutionPolicy Bypass -File .\sdks\sdkwork-audio-app-sdk\bin\generate-sdk.ps1 -Languages typescript

# Generate Backend SDK
powershell -NoProfile -ExecutionPolicy Bypass -File .\sdks\sdkwork-audio-backend-sdk\bin\generate-sdk.ps1 -Languages typescript
```

### 4.2 Generation Manifest

```json
{
  "generator": "@sdkwork/sdk-generator",
  "generatorVersion": "1.0.0",
  "cli": "sdkgen",
  "inputSpec": "sdks/sdkwork-audio-app-sdk/openapi/sdkwork-audio-app-api.openapi.yaml",
  "outputDir": "sdks/sdkwork-audio-app-sdk/sdkwork-audio-app-sdk-typescript/generated/server-openapi",
  "language": "typescript",
  "sdkType": "app-api",
  "packageName": "@sdkwork/audio-app-sdk",
  "standardProfile": "sdkwork-v3"
}
```

### 4.3 SDK Assembly Metadata

**File**: `sdk-manifest.json`

```json
{
  "sdkOwner": "sdkwork-audio",
  "apiAuthority": "sdkwork-audio-app-api",
  "sdkFamily": "sdkwork-audio-app-sdk",
  "generationInputSpec": "sdks/sdkwork-audio-app-sdk/openapi/sdkwork-audio-app-api.openapi.yaml",
  "standardProfile": "sdkwork-v3",
  "sdkDependencies": [
    {
      "workspace": "clawrouter-open-sdk",
      "role": "ai-audio-generation-provider-capability",
      "required": true,
      "dependencyMode": "consumer-sdk",
      "apiPrefix": null,
      "generatedTransportImportPolicy": "forbidden",
      "packageByLanguage": {
        "typescript": "@sdkwork/clawrouter-open-sdk"
      }
    },
    {
      "workspace": "sdkwork-drive-sdk",
      "role": "audio-storage-capability",
      "required": true,
      "dependencyMode": "consumer-sdk",
      "apiPrefix": null,
      "generatedTransportImportPolicy": "forbidden",
      "packageByLanguage": {
        "typescript": "@sdkwork/drive-sdk"
      }
    },
    {
      "workspace": "sdkwork-iam-sdk",
      "role": "authentication-capability",
      "required": true,
      "dependencyMode": "consumer-sdk",
      "apiPrefix": null,
      "generatedTransportImportPolicy": "forbidden",
      "packageByLanguage": {
        "typescript": "@sdkwork/iam-sdk"
      }
    }
  ]
}
```

## 5. SDK Usage Examples

### 5.1 App SDK Usage

```typescript
import { AudioAppClient } from '@sdkwork/audio-app-sdk';

// Initialize client
const client = new AudioAppClient({
  baseUrl: 'https://api.sdkwork.com',
  accessToken: 'your-access-token',
  authorization: 'your-jwt-token'
});

// Speech synthesis
const speechResult = await client.speech.create({
  text: 'Hello, this is a test of the speech synthesis system.',
  voiceId: 'en-us-male-1',
  speed: 1.0,
  pitch: 1.0,
  emotion: 'neutral',
  audioFormat: 'mp3'
});

console.log('Task ID:', speechResult.data.taskId);

// List voices
const voices = await client.speech.voices.list({
  language: 'en',
  limit: 20
});

console.log('Voices:', voices.data.items);

// Create transcription
const transcriptionResult = await client.transcriptions.create({
  audioUrl: 'https://example.com/audio.mp3',
  language: 'en',
  enableTimestamps: true,
  enableSpeakerDiarization: true
});

console.log('Transcription Task ID:', transcriptionResult.data.taskId);

// Get task status
const task = await client.tasks.retrieve(speechResult.data.taskId);
console.log('Task Status:', task.data.status);
console.log('Artifacts:', task.data.artifacts);

// Real-time session
const session = await client.realtime.sessions.create({
  sessionType: 'transcription',
  language: 'en',
  enableTranslation: true,
  targetLanguage: 'zh'
});

console.log('WebSocket URL:', session.data.websocketUrl);

// Create workspace
const workspace = await client.workspaces.create({
  name: 'My Audio Project',
  description: 'A workspace for my audio project'
});

console.log('Workspace ID:', workspace.data.workspaceId);

// Export audio
const exportResult = await client.exports.create({
  workspaceId: workspace.data.workspaceId,
  format: 'mp3',
  quality: 'high',
  sampleRate: 44100
});

console.log('Export Task ID:', exportResult.data.taskId);
```

### 5.2 Backend SDK Usage

```typescript
import { AudioBackendClient } from '@sdkwork/audio-backend-sdk';

// Initialize admin client
const adminClient = new AudioBackendClient({
  baseUrl: 'https://api.sdkwork.com',
  accessToken: 'admin-access-token',
  authorization: 'admin-jwt-token'
});

// Create provider route
const providerRoute = await adminClient.providerRoutes.create({
  providerCode: 'openai',
  providerName: 'OpenAI',
  providerType: 'openai',
  baseUrl: 'https://api.openai.com/v1',
  apiKey: 'sk-...',
  capabilities: ['speech', 'transcription', 'translation'],
  config: {
    model: 'tts-1',
    voice: 'alloy'
  }
});

console.log('Provider Route ID:', providerRoute.data.providerRouteId);

// List all tasks
const tasks = await adminClient.tasks.list({
  limit: 100,
  status: 'failed'
});

console.log('Failed Tasks:', tasks.data.items);

// Retry failed task
const retryResult = await adminClient.tasks.retry(tasks.data.items[0].taskId);
console.log('Retry Result:', retryResult.data);

// Get usage analytics
const analytics = await adminClient.analytics.usage({
  startDate: '2026-06-01',
  endDate: '2026-06-14',
  groupBy: 'day',
  operationType: 'speech'
});

console.log('Usage Analytics:', analytics.data);

// List webhook events
const webhookEvents = await adminClient.webhookEvents.list({
  limit: 50
});

console.log('Webhook Events:', webhookEvents.data.items);

// Replay webhook event
const replayResult = await adminClient.webhookEvents.replay(
  webhookEvents.data.items[0].eventId
);

console.log('Replay Result:', replayResult.data);
```

### 5.3 Real-time WebSocket Usage

```typescript
import { AudioAppClient } from '@sdkwork/audio-app-sdk';

const client = new AudioAppClient({
  baseUrl: 'https://api.sdkwork.com',
  accessToken: 'your-access-token',
  authorization: 'your-jwt-token'
});

// Create real-time session
const session = await client.realtime.sessions.create({
  sessionType: 'transcription',
  language: 'en',
  enableTranslation: true,
  targetLanguage: 'zh'
});

// Connect to WebSocket
const ws = new WebSocket(session.data.websocketUrl);

ws.onopen = () => {
  console.log('Connected to real-time session');
  
  // Send audio data
  ws.send(JSON.stringify({
    type: 'audio',
    data: audioBuffer
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  
  switch (message.type) {
    case 'transcript':
      console.log('Transcript:', message.text);
      console.log('Speaker:', message.speakerId);
      console.log('Timestamp:', message.startMs, '-', message.endMs);
      break;
    
    case 'translation':
      console.log('Translation:', message.translatedText);
      console.log('Source:', message.sourceText);
      break;
    
    case 'status':
      console.log('Status:', message.status);
      break;
    
    case 'error':
      console.error('Error:', message.error);
      break;
  }
};

ws.onclose = () => {
  console.log('Disconnected from real-time session');
};
```

## 6. SDK Configuration

### 6.1 Client Configuration

```typescript
interface AudioAppClientConfig {
  // Base URL for the API
  baseUrl: string;
  
  // Authentication tokens
  accessToken: string;
  authorization: string;
  
  // Optional configuration
  timeout?: number; // Request timeout in ms (default: 30000)
  retries?: number; // Number of retries (default: 3)
  retryDelay?: number; // Delay between retries in ms (default: 1000)
  
  // Custom headers
  headers?: Record<string, string>;
  
  // Request interceptor
  onRequest?: (request: Request) => Request | Promise<Request>;
  
  // Response interceptor
  onResponse?: (response: Response) => Response | Promise<Response>;
  
  // Error handler
  onError?: (error: Error) => void;
}
```

### 6.2 Environment Configuration

```typescript
// Development
const devConfig = {
  baseUrl: 'http://localhost:3000',
  accessToken: process.env.DEV_ACCESS_TOKEN,
  authorization: process.env.DEV_AUTHORIZATION
};

// Staging
const stagingConfig = {
  baseUrl: 'https://staging-api.sdkwork.com',
  accessToken: process.env.STAGING_ACCESS_TOKEN,
  authorization: process.env.STAGING_AUTHORIZATION
};

// Production
const prodConfig = {
  baseUrl: 'https://api.sdkwork.com',
  accessToken: process.env.PROD_ACCESS_TOKEN,
  authorization: process.env.PROD_AUTHORIZATION
};
```

## 7. Error Handling

### 7.1 Error Types

```typescript
class AudioSDKError extends Error {
  constructor(
    message: string,
    public code: string,
    public status: number,
    public details?: any
  ) {
    super(message);
    this.name = 'AudioSDKError';
  }
}

class ValidationError extends AudioSDKError {
  constructor(message: string, details?: any) {
    super(message, 'VALIDATION_ERROR', 400, details);
    this.name = 'ValidationError';
  }
}

class AuthenticationError extends AudioSDKError {
  constructor(message: string) {
    super(message, 'AUTHENTICATION_ERROR', 401);
    this.name = 'AuthenticationError';
  }
}

class RateLimitError extends AudioSDKError {
  constructor(message: string, public retryAfter: number) {
    super(message, 'RATE_LIMIT_ERROR', 429);
    this.name = 'RateLimitError';
  }
}

class ProviderError extends AudioSDKError {
  constructor(message: string, public providerCode: string) {
    super(message, 'PROVIDER_ERROR', 502);
    this.name = 'ProviderError';
  }
}
```

### 7.2 Error Handling Example

```typescript
import { AudioAppClient, ValidationError, RateLimitError } from '@sdkwork/audio-app-sdk';

const client = new AudioAppClient(config);

try {
  const result = await client.speech.create({
    text: 'Hello, world!',
    voiceId: 'en-us-male-1'
  });
  console.log('Success:', result.data);
} catch (error) {
  if (error instanceof ValidationError) {
    console.error('Validation failed:', error.details);
  } else if (error instanceof RateLimitError) {
    console.error(`Rate limited. Retry after ${error.retryAfter} seconds`);
    // Implement retry logic
  } else {
    console.error('Unexpected error:', error);
  }
}
```

## 8. SDK Dependencies

### 8.1 Dependency Declaration

```json
{
  "sdkDependencies": [
    {
      "workspace": "clawrouter-open-sdk",
      "role": "ai-audio-generation-provider-capability",
      "required": true,
      "dependencyMode": "consumer-sdk",
      "apiPrefix": null,
      "generatedTransportImportPolicy": "forbidden",
      "packageByLanguage": {
        "typescript": "@sdkwork/clawrouter-open-sdk"
      }
    },
    {
      "workspace": "sdkwork-drive-sdk",
      "role": "audio-storage-capability",
      "required": true,
      "dependencyMode": "consumer-sdk",
      "apiPrefix": null,
      "generatedTransportImportPolicy": "forbidden",
      "packageByLanguage": {
        "typescript": "@sdkwork/drive-sdk"
      }
    },
    {
      "workspace": "sdkwork-iam-sdk",
      "role": "authentication-capability",
      "required": true,
      "dependencyMode": "consumer-sdk",
      "apiPrefix": null,
      "generatedTransportImportPolicy": "forbidden",
      "packageByLanguage": {
        "typescript": "@sdkwork/iam-sdk"
      }
    }
  ]
}
```

### 8.2 Dependency Usage

```typescript
import { AudioAppClient } from '@sdkwork/audio-app-sdk';
import { DriveClient } from '@sdkwork/drive-sdk';
import { IAMClient } from '@sdkwork/iam-sdk';

// Initialize IAM client for authentication
const iamClient = new IAMClient({
  baseUrl: 'https://api.sdkwork.com'
});

// Get access token
const token = await iamClient.auth.login({
  username: 'user@example.com',
  password: 'password'
});

// Initialize Audio client with token
const audioClient = new AudioAppClient({
  baseUrl: 'https://api.sdkwork.com',
  accessToken: token.data.accessToken,
  authorization: token.data.authorization
});

// Initialize Drive client for file operations
const driveClient = new DriveClient({
  baseUrl: 'https://api.sdkwork.com',
  accessToken: token.data.accessToken,
  authorization: token.data.authorization
});

// Upload audio file to Drive
const uploadResult = await driveClient.upload({
  file: audioFile,
  spaceType: 'ai_generated'
});

// Use uploaded file for transcription
const transcription = await audioClient.transcriptions.create({
  audioUrl: uploadResult.data.downloadUrl,
  language: 'en'
});
```

## 9. SDK Verification

### 9.1 Verification Commands

```powershell
# Typecheck
pnpm typecheck

# Build
pnpm build

# Test
pnpm test

# Lint
pnpm lint

# Verify generated SDK
node .\bin\publish-core.mjs --language typescript --project-dir . --action check
node .\bin\publish-core.mjs --language typescript --project-dir . --action build
```

### 9.2 Verification Checklist

- [ ] SDK generates without warnings
- [ ] All operations are accessible
- [ ] Authentication works correctly
- [ ] Error handling works as expected
- [ ] Dependencies are properly declared
- [ ] TypeScript types are correct
- [ ] Documentation is complete

## 10. SDK Publishing

### 10.1 Publishing Process

1. **Generate SDK**: Run SDK generation command
2. **Verify SDK**: Run verification commands
3. **Build SDK**: Build the SDK package
4. **Test SDK**: Run SDK tests
5. **Publish SDK**: Publish to npm registry

### 10.2 Versioning

```json
{
  "name": "@sdkwork/audio-app-sdk",
  "version": "1.0.0",
  "description": "SDKWork Audio App SDK",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": [
    "dist"
  ],
  "scripts": {
    "build": "tsc",
    "test": "jest",
    "prepublishOnly": "npm run build"
  }
}
```

---

*Document generated by SDKWork Audio SDK Design System*
*Version: 1.0*
*Date: June 14, 2026*
