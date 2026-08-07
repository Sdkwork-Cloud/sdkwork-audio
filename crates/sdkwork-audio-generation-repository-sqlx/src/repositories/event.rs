//! Event repository

use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::event::*;

/// Event repository trait
#[async_trait]
pub trait EventRepository {
    async fn create(&self, request: CreateEventRequest) -> Result<AudioTaskEvent, sqlx::Error>;
    async fn get_by_id(&self, id: i64) -> Result<Option<AudioTaskEvent>, sqlx::Error>;
    async fn get_by_event_no(&self, event_no: &str) -> Result<Option<AudioTaskEvent>, sqlx::Error>;
    async fn list_by_task(&self, task_id: i64, limit: i64, offset: i64) -> Result<EventListResult, sqlx::Error>;
    async fn list(&self, filter: EventFilter, limit: i64, offset: i64) -> Result<EventListResult, sqlx::Error>;
    async fn count(&self, filter: EventFilter) -> Result<i64, sqlx::Error>;
}



