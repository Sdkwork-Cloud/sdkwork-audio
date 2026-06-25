> Migrated from `docs/TRANSLATION_SERVICE_IMPLEMENTATION.md` on 2026-06-24.
> Owner: SDKWork maintainers

## Overview

This document describes the implementation of the translation service, which provides end-to-end audio translation functionality.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    API Layer                                  │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  sdkwork-router-audio-app-api                          │ │
│  │  - POST /app/v3/api/audio/translations                 │ │
│  │  - GET /app/v3/api/audio/translations/{taskId}         │ │
│  │  - GET /app/v3/api/audio/translations/languages        │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Service Layer                              │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  sdkwork-audio-translation-service                     │ │
│  │  - TranslationService trait                            │ │
│  │  - TranslationServiceImpl                              │ │
│  │  - Request/Response models                             │ │
│  │  - Error handling                                      │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│  Repository     │ │  AI Engine      │ │  Drive Service  │
│  Layer          │ │  Integration    │ │  Integration    │
│                 │ │                 │ │                 │
│  - TaskRepo     │ │  - OpenAI       │ │  - Upload       │
│  - ArtifactRepo │ │  - Azure        │ │  - Download     │
│                 │ │  - Google       │ │                 │
└─────────────────┘ └─────────────────┘ └─────────────────┘
```

## Components

### 1. Service Layer

#### Translation Service (`sdkwork-audio-translation-service`)
- **File**: `src/service.rs`
- **Trait**: `TranslationService`
- **Implementation**: `TranslationServiceImpl`
- **Operations**:
  - `create_translation()` - Create translation task
  - `get_translation_result()` - Get translation result
  - `get_translation_detail()` - Get task detail
  - `list_translations()` - List translation tasks
  - `cancel_translation()` - Cancel translation task
  - `get_supported_languages()` - Get supported languages

### 2. Models

#### Request Models
- `TranslationRequest` - Translation request parameters
- `TranslationListRequest` - List request parameters

#### Response Models
- `TranslationResponse` - Translation task response
- `TranslationResult` - Translation result
- `TranslationTaskDetail` - Task detail
- `TranslationListResponse` - List response
- `SupportedLanguagesResponse` - Supported languages

#### Data Models
- `TranslationSegment` - Translation segment
- `OutputFormat` - Output format (JSON, SRT, VTT, TXT)
- `SupportedLanguage` - Supported language

### 3. Error Handling

#### Error Types
- `TranslationServiceError::Database` - Database errors
- `TranslationServiceError::AiEngine` - AI engine errors
- `TranslationServiceError::DriveService` - Drive service errors
- `TranslationServiceError::TaskNotFound` - Task not found
- `TranslationServiceError::InvalidRequest` - Invalid request
- `TranslationServiceError::ProviderError` - Provider errors
- `TranslationServiceError::TaskAlreadyExists` - Duplicate task
- `TranslationServiceError::UnsupportedLanguage` - Unsupported language
- `TranslationServiceError::AudioDownloadError` - Audio download error
- `TranslationServiceError::Internal` - Internal errors

## Data Flow

### Translation Flow

1. **Client Request**
   - Client sends POST request to `/app/v3/api/audio/translations`
   - Request includes audio URL, source language, target language, output format, etc.

2. **API Layer**
   - Validates request
   - Creates TranslationRequest
   - Calls TranslationService.create_translation()

3. **Service Layer**
   - Checks idempotency key
   - Validates audio URL and target language
   - Calculates input hash
   - Creates task in database (status: queued)
   - Spawns async processing task
   - Returns task ID and status

4. **Async Processing**
   - Updates task status to running
   - Calls AI engine to translate audio
   - Formats output based on requested format
   - Uploads output to drive storage
   - Creates artifact record
   - Updates task status to succeeded
   - Stores result metadata

5. **Client Polling**
   - Client polls GET `/app/v3/api/audio/tasks/{taskId}`
   - Returns task status and result when complete

## Supported Languages

| Code | Name | Native Name |
|------|------|-------------|
| en | English | English |
| zh | Chinese | 中文 |
| ja | Japanese | 日本語 |
| ko | Korean | 한국어 |
| es | Spanish | Español |
| fr | French | Français |
| de | German | Deutsch |
| it | Italian | Italiano |
| pt | Portuguese | Português |
| ru | Russian | Русский |
| ar | Arabic | العربية |
| hi | Hindi | हिन्दी |

## Output Formats

### JSON Format
```json
{
  "sourceText": "Original text",
  "translatedText": "Translated text",
  "sourceLanguage": "en",
  "targetLanguage": "zh",
  "confidence": 0.95
}
```

### SRT Format
```
1
00:00:00,000 --> 00:00:10,000
Translated text
```

### VTT Format
```WEBVTT

00:00:00.000 --> 00:00:10.000
Translated text
```

### TXT Format
```
Translated text without timestamps
```

## Database Schema

### Tables Used

1. **audio_generation_task** - Stores task information
2. **audio_audio_artifact** - Stores artifact information

### Key Fields

- **task_no** - UUID for task identification
- **idempotency_key** - For request deduplication
- **status** - Task status (queued, running, succeeded, failed, cancelled)
- **progress** - Task progress (0-100)
- **result_json** - Task result metadata

## AI Engine Integration

### Supported Providers

1. **OpenAI Whisper**
   - Multi-language support
   - Speech-to-text + translation

2. **Azure Speech Services**
   - Real-time translation
   - Batch translation

3. **Google Cloud Speech-to-Text**
   - Multi-language support
   - Speech translation

## Testing

### Unit Tests

- Translation service tests
- Output format tests
- Error handling tests
- Language support tests

### Integration Tests

- End-to-end translation flow
- Multiple output formats
- Multiple language pairs
- Error scenarios

### Contract Tests

- API contract validation
- SDK generation validation

## Configuration

### Environment Variables

- `DATABASE_URL` - Database connection URL
- `DRIVE_BASE_URL` - Drive service base URL
- `IAM_BASE_URL` - IAM service base URL

### Feature Flags

- `ENABLE_AUTO_DETECT` - Enable auto language detection
- `ENABLE_TIMESTAMPS` - Enable timestamp preservation
- `MAX_AUDIO_DURATION` - Maximum audio duration in seconds

## Monitoring

### Metrics

- Translation request count
- Translation latency
- Error rate
- Language pair distribution
- Output format distribution

### Logging

- Request/response logging
- Error logging
- Performance logging

## Future Enhancements

1. **Real-time Translation** - Live audio stream translation
2. **Speech-to-Speech Translation** - Direct audio translation
3. **Custom Glossary** - Domain-specific term translation
4. **Translation Memory** - Reuse previous translations
5. **Batch Processing** - Bulk translation
6. **Quality Metrics** - Translation quality assessment
7. **Multi-channel Support** - Multiple audio channels
8. **Dialect Support** - Regional dialect translation

---

*Implementation documented by SDKWork Audio Translation Service*
*Version: 1.0*
*Date: June 14, 2026*

