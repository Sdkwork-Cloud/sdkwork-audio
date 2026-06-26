//! SDKWork Audio app API route definitions
//!
//! This crate defines the route catalog for the Audio app API.

use serde::{Deserialize, Serialize};

/// API operation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOperation {
    pub operation_id: String,
    pub method: String,
    pub path: String,
    pub summary: String,
    pub description: String,
    pub tags: Vec<String>,
    pub requires_auth: bool,
}

/// Route catalog for Audio app API
pub struct AudioAppApiRouteCatalog;

impl AudioAppApiRouteCatalog {
    /// Get all operations
    pub fn operations() -> Vec<ApiOperation> {
        vec![
            // Speech synthesis operations
            ApiOperation {
                operation_id: "speech.create".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/speech".to_string(),
                summary: "Create speech synthesis task".to_string(),
                description: "Create a new speech synthesis task from text input.".to_string(),
                tags: vec!["speech".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "speech.voices.list".to_string(),
                method: "GET".to_string(),
                path: "/app/v3/api/audio/speech/voices".to_string(),
                summary: "List speech voices".to_string(),
                description: "List available speech synthesis voices.".to_string(),
                tags: vec!["speech".to_string()],
                requires_auth: true,
            },
            // Transcription operations
            ApiOperation {
                operation_id: "transcriptions.create".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/transcriptions".to_string(),
                summary: "Create transcription task".to_string(),
                description: "Create a new audio transcription task.".to_string(),
                tags: vec!["transcriptions".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "transcriptions.segments.list".to_string(),
                method: "GET".to_string(),
                path: "/app/v3/api/audio/transcriptions/{taskId}/segments".to_string(),
                summary: "List transcription segments".to_string(),
                description: "List segments of a completed transcription.".to_string(),
                tags: vec!["transcriptions".to_string()],
                requires_auth: true,
            },
            // Translation operations
            ApiOperation {
                operation_id: "translations.create".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/translations".to_string(),
                summary: "Create translation task".to_string(),
                description: "Create a new audio translation task.".to_string(),
                tags: vec!["translations".to_string()],
                requires_auth: true,
            },
            // Sound effect operations
            ApiOperation {
                operation_id: "soundEffects.create".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/sound-effects".to_string(),
                summary: "Generate sound effect".to_string(),
                description: "Generate a sound effect from text description.".to_string(),
                tags: vec!["soundEffects".to_string()],
                requires_auth: true,
            },
            // Task management operations
            ApiOperation {
                operation_id: "tasks.list".to_string(),
                method: "GET".to_string(),
                path: "/app/v3/api/audio/tasks".to_string(),
                summary: "List tasks".to_string(),
                description: "List user's audio generation tasks.".to_string(),
                tags: vec!["tasks".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "tasks.retrieve".to_string(),
                method: "GET".to_string(),
                path: "/app/v3/api/audio/tasks/{taskId}".to_string(),
                summary: "Get task details".to_string(),
                description: "Get details of a specific task.".to_string(),
                tags: vec!["tasks".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "tasks.cancel".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/tasks/{taskId}/cancel".to_string(),
                summary: "Cancel task".to_string(),
                description: "Cancel a running task.".to_string(),
                tags: vec!["tasks".to_string()],
                requires_auth: true,
            },
            // Real-time operations
            ApiOperation {
                operation_id: "realtime.sessions.create".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/realtime/sessions".to_string(),
                summary: "Create real-time session".to_string(),
                description: "Create a new real-time audio processing session.".to_string(),
                tags: vec!["realtime".to_string()],
                requires_auth: true,
            },
            // Voice management operations
            ApiOperation {
                operation_id: "voices.list".to_string(),
                method: "GET".to_string(),
                path: "/app/v3/api/audio/voices".to_string(),
                summary: "List voices".to_string(),
                description: "List user's custom voices.".to_string(),
                tags: vec!["voices".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "voices.create".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/voices".to_string(),
                summary: "Create voice".to_string(),
                description: "Create a custom voice from reference audio.".to_string(),
                tags: vec!["voices".to_string()],
                requires_auth: true,
            },
            // Workspace operations
            ApiOperation {
                operation_id: "workspaces.list".to_string(),
                method: "GET".to_string(),
                path: "/app/v3/api/audio/workspaces".to_string(),
                summary: "List workspaces".to_string(),
                description: "List user's audio workspaces.".to_string(),
                tags: vec!["workspaces".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "workspaces.create".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/workspaces".to_string(),
                summary: "Create workspace".to_string(),
                description: "Create a new audio workspace.".to_string(),
                tags: vec!["workspaces".to_string()],
                requires_auth: true,
            },
            // Export operations
            ApiOperation {
                operation_id: "exports.create".to_string(),
                method: "POST".to_string(),
                path: "/app/v3/api/audio/exports".to_string(),
                summary: "Create export".to_string(),
                description: "Create an audio export task.".to_string(),
                tags: vec!["exports".to_string()],
                requires_auth: true,
            },
        ]
    }

    /// Get operation by ID
    pub fn get_operation(operation_id: &str) -> Option<ApiOperation> {
        Self::operations().into_iter().find(|op| op.operation_id == operation_id)
    }

    /// Get operations by tag
    pub fn get_operations_by_tag(tag: &str) -> Vec<ApiOperation> {
        Self::operations()
            .into_iter()
            .filter(|op| op.tags.contains(&tag.to_string()))
            .collect()
    }
}

/// API prefix
pub const API_PREFIX: &str = "/app/v3/api/audio";

/// API version
pub const API_VERSION: &str = "v3";

/// Get full path for operation
pub fn get_full_path(path: &str) -> String {
    format!("{}{}", API_PREFIX, path)
}

pub fn gateway_mount() -> axum::Router {
    axum::Router::new()
}
