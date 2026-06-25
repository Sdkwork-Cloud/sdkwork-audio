//! Audio task event entity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum EventType {
    StatusChange,
    ProgressUpdate,
    Error,
    Retry,
    Webhook,
    Cancel,
    Reconcile,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::StatusChange => "status_change",
            EventType::ProgressUpdate => "progress_update",
            EventType::Error => "error",
            EventType::Retry => "retry",
            EventType::Webhook => "webhook",
            EventType::Cancel => "cancel",
            EventType::Reconcile => "reconcile",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "status_change" => Some(EventType::StatusChange),
            "progress_update" => Some(EventType::ProgressUpdate),
            "error" => Some(EventType::Error),
            "retry" => Some(EventType::Retry),
            "webhook" => Some(EventType::Webhook),
            "cancel" => Some(EventType::Cancel),
            "reconcile" => Some(EventType::Reconcile),
            _ => None,
        }
    }
}

/// Audio task event entity
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioTaskEvent {
    pub id: i64,
    pub event_no: String,
    pub task_id: i64,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: Option<String>,
    pub provider_code: Option<String>,
    pub provider_event_id: Option<String>,
    pub provider_task_id: Option<String>,
    pub payload_hash: Option<String>,
    pub payload_json: String,
    pub received_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub status: String,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Create event request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub task_id: i64,
    pub event_type: EventType,
    pub from_status: Option<String>,
    pub to_status: Option<String>,
    pub provider_code: Option<String>,
    pub provider_event_id: Option<String>,
    pub provider_task_id: Option<String>,
    pub payload_json: String,
    pub message: Option<String>,
}

/// Event filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    pub task_id: Option<i64>,
    pub event_type: Option<EventType>,
    pub provider_code: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
}

/// Event list result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventListResult {
    pub events: Vec<AudioTaskEvent>,
    pub total: i64,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}
