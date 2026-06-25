# SDKWork Audio - Transcription Service Implementation

## Overview

This document describes the implementation of the transcription service, which provides end-to-end audio-to-text functionality.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    API Layer                                  │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  sdkwork-router-audio-app-api                          │ │
│  │  - POST /app/v3/api/audio/transcriptions               │ │
│  │  - GET /app/v3/api/audio/transcriptions/{taskId}       │ │
│  │  - GET /app/v3/api/audio/transcriptions/{taskId}/segments│
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Service Layer                              │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  sdkwork-audio-transcription-service                   │ │
│  │  - TranscriptionService trait                          │ │
│  │  - TranscriptionServiceImpl                            │ │
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

#### Transcription Service (`sdkwork-audio-transcription-service`)
- **File**: `src/service.rs`
- **Trait**: `TranscriptionService`
- **Implementation**: `TranscriptionServiceImpl`
- **Operations**:
  - `create_transcription()` - Create transcription task
  - `get_transcription_result()` - Get transcription result
  - `get_transcription_detail()` - Get task detail
  - `list_transcriptions()` - List transcription tasks
  - `cancel_transcription()` - Cancel transcription task

### 2. Models

#### Request Models
- `TranscriptionRequest` - Transcription request parameters
- `TranscriptionListRequest` - List request parameters

#### Response Models
- `TranscriptionResponse` - Transcription task response
- `TranscriptionResult` - Transcription result
- `TranscriptionTaskDetail` - Task detail
- `TranscriptionListResponse` - List response

#### Data Models
- `TranscriptionSegment` - Transcription segment
- `TranscriptionWord` - Word-level transcription
- `OutputFormat` - Output format (JSON, SRT, VTT, TXT)

### 3. Error Handling

#### Error Types
- `TranscriptionServiceError::Database` - Database errors
- `TranscriptionServiceError::AiEngine` - AI engine errors
- `TranscriptionServiceError::DriveService` - Drive service errors
- `TranscriptionServiceError::TaskNotFound` - Task not found
- `TranscriptionServiceError::InvalidRequest` - Invalid request
- `TranscriptionServiceError::ProviderError` - Provider errors
- `TranscriptionServiceError::TaskAlreadyExists` - Duplicate task
- `TranscriptionServiceError::AudioDownloadError` - Audio download error
- `TranscriptionServiceError::Internal` - Internal errors

## Data Flow

### Transcription Flow

1. **Client Request**
   - Client sends POST request to `/app/v3/api/audio/transcriptions`
   - Request includes audio URL, language, output format, etc.

2. **API Layer**
   - Validates request
   - Creates TranscriptionRequest
   - Calls TranscriptionService.create_transcription()

3. **Service Layer**
   - Checks idempotency key
   - Validates audio URL
   - Calculates input hash
   - Creates task in database (status: queued)
   - Spawns async processing task
   - Returns task ID and status

4. **Async Processing**
   - Updates task status to running
   - Calls AI engine to transcribe audio
   - Formats output based on requested format
   - Uploads output to drive storage
   - Creates artifact record
   - Updates task status to succeeded
   - Stores result metadata

5. **Client Polling**
   - Client polls GET `/app/v3/api/audio/tasks/{taskId}`
   - Returns task status and result when complete

## Output Formats

### JSON Format
```json
{
  "text": "Full transcription text",
  "language": "en",
  "confidence": 0.95,
  "segments": [
    {
      "startMs": 0,
      "endMs": 5000,
      "text": "Segment text",
      "speakerId": 1,
      "confidence": 0.98
    }
  ]
}
```

### SRT Format
```
1
00:00:00,000 --> 00:00:05,000
Segment text

2
00:00:05,000 --> 00:00:10,000
Next segment text
```

### VTT Format
```WEBVTT

00:00:00.000 --> 00:00:05.000
Segment text

00:00:05.000 --> 00:00:10.000
Next segment text
```

### TXT Format
```
Full transcription text without timestamps
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
   - Speaker diarization
   - Timestamp alignment

2. **Azure Speech Services**
   - Real-time transcription
   - Batch transcription
   - Custom vocabulary

3. **Google Cloud Speech-to-Text**
   - Multi-language support
   - Speaker diarization
   - Word-level timestamps

## Testing

### Unit Tests

- Transcription service tests
- Output format tests
- Error handling tests

### Integration Tests

- End-to-end transcription flow
- Multiple output formats
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

- `ENABLE_SPEAKER_DIARIZATION` - Enable speaker diarization
- `ENABLE_WORD_TIMESTAMPS` - Enable word-level timestamps
- `MAX_SPEAKERS` - Maximum number of speakers

## Monitoring

### Metrics

- Transcription request count
- Transcription latency
- Error rate
- Output format distribution

### Logging

- Request/response logging
- Error logging
- Performance logging

## Future Enhancements

1. **Real-time Transcription** - Live audio stream transcription
2. **Custom Vocabulary** - Domain-specific term recognition
3. **Speaker Identification** - Speaker voice matching
4. **Translation Integration** - Transcription + translation
5. **Batch Processing** - Bulk transcription
6. **Quality Metrics** - Transcription accuracy assessment
7. **Multi-channel Support** - Multiple audio channels
8. **Noise Reduction** - Audio preprocessing

---

*Implementation documented by SDKWork Audio Transcription Service*
*Version: 1.0*
*Date: June 14, 2026*
