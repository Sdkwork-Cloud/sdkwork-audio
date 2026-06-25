//! SDKWork Audio backend API route definitions
//!
//! This crate defines the route catalog for the Audio backend API.

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

/// Route catalog for Audio backend API
pub struct AudioBackendApiRouteCatalog;

impl AudioBackendApiRouteCatalog {
    /// Get all operations
    pub fn operations() -> Vec<ApiOperation> {
        vec![
            // Provider route operations
            ApiOperation {
                operation_id: "providerRoutes.list".to_string(),
                method: "GET".to_string(),
                path: "/backend/v3/api/audio/provider-routes".to_string(),
                summary: "List provider routes".to_string(),
                description: "List all provider routes.".to_string(),
                tags: vec!["providerRoutes".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "providerRoutes.create".to_string(),
                method: "POST".to_string(),
                path: "/backend/v3/api/audio/provider-routes".to_string(),
                summary: "Create provider route".to_string(),
                description: "Create a new provider route.".to_string(),
                tags: vec!["providerRoutes".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "providerRoutes.retrieve".to_string(),
                method: "GET".to_string(),
                path: "/backend/v3/api/audio/provider-routes/{providerRouteId}".to_string(),
                summary: "Get provider route".to_string(),
                description: "Get details of a specific provider route.".to_string(),
                tags: vec!["providerRoutes".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "providerRoutes.update".to_string(),
                method: "PATCH".to_string(),
                path: "/backend/v3/api/audio/provider-routes/{providerRouteId}".to_string(),
                summary: "Update provider route".to_string(),
                description: "Update a provider route.".to_string(),
                tags: vec!["providerRoutes".to_string()],
                requires_auth: true,
            },
            // Task administration operations
            ApiOperation {
                operation_id: "tasks.list".to_string(),
                method: "GET".to_string(),
                path: "/backend/v3/api/audio/tasks".to_string(),
                summary: "List tasks".to_string(),
                description: "List all tasks (admin).".to_string(),
                tags: vec!["tasks".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "tasks.retrieve".to_string(),
                method: "GET".to_string(),
                path: "/backend/v3/api/audio/tasks/{taskId}".to_string(),
                summary: "Get task details".to_string(),
                description: "Get details of a specific task (admin).".to_string(),
                tags: vec!["tasks".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "tasks.retry".to_string(),
                method: "POST".to_string(),
                path: "/backend/v3/api/audio/tasks/{taskId}/retry".to_string(),
                summary: "Retry task".to_string(),
                description: "Retry a failed task.".to_string(),
                tags: vec!["tasks".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "tasks.reconcile".to_string(),
                method: "POST".to_string(),
                path: "/backend/v3/api/audio/tasks/{taskId}/reconcile".to_string(),
                summary: "Reconcile task".to_string(),
                description: "Reconcile a stuck task.".to_string(),
                tags: vec!["tasks".to_string()],
                requires_auth: true,
            },
            // Webhook management operations
            ApiOperation {
                operation_id: "providerWebhooks.accept".to_string(),
                method: "POST".to_string(),
                path: "/backend/v3/api/audio/provider-webhooks/{providerCode}".to_string(),
                summary: "Accept webhook".to_string(),
                description: "Accept a provider webhook.".to_string(),
                tags: vec!["webhooks".to_string()],
                requires_auth: false,
            },
            ApiOperation {
                operation_id: "webhookEvents.list".to_string(),
                method: "GET".to_string(),
                path: "/backend/v3/api/audio/webhook-events".to_string(),
                summary: "List webhook events".to_string(),
                description: "List webhook events.".to_string(),
                tags: vec!["webhooks".to_string()],
                requires_auth: true,
            },
            ApiOperation {
                operation_id: "webhookEvents.replay".to_string(),
                method: "POST".to_string(),
                path: "/backend/v3/api/audio/webhook-events/{eventId}/replay".to_string(),
                summary: "Replay webhook event".to_string(),
                description: "Replay a webhook event.".to_string(),
                tags: vec!["webhooks".to_string()],
                requires_auth: true,
            },
            // Analytics operations
            ApiOperation {
                operation_id: "analytics.usage".to_string(),
                method: "GET".to_string(),
                path: "/backend/v3/api/audio/analytics/usage".to_string(),
                summary: "Get usage analytics".to_string(),
                description: "Get usage analytics.".to_string(),
                tags: vec!["analytics".to_string()],
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
pub const API_PREFIX: &str = "/backend/v3/api/audio";

/// API version
pub const API_VERSION: &str = "v3";

/// Get full path for operation
pub fn get_full_path(path: &str) -> String {
    format!("{}{}", API_PREFIX, path)
}
