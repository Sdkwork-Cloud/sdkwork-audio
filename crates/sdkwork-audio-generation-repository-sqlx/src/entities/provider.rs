//! Audio provider entity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Provider type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum ProviderType {
    OpenAI,
    ElevenLabs,
    Azure,
    Google,
    Custom,
}

impl ProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderType::OpenAI => "openai",
            ProviderType::ElevenLabs => "elevenlabs",
            ProviderType::Azure => "azure",
            ProviderType::Google => "google",
            ProviderType::Custom => "custom",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "openai" => Some(ProviderType::OpenAI),
            "elevenlabs" => Some(ProviderType::ElevenLabs),
            "azure" => Some(ProviderType::Azure),
            "google" => Some(ProviderType::Google),
            "custom" => Some(ProviderType::Custom),
            _ => None,
        }
    }
}

/// Provider status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum ProviderStatus {
    Active,
    Inactive,
    Maintenance,
}

impl ProviderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderStatus::Active => "active",
            ProviderStatus::Inactive => "inactive",
            ProviderStatus::Maintenance => "maintenance",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(ProviderStatus::Active),
            "inactive" => Some(ProviderStatus::Inactive),
            "maintenance" => Some(ProviderStatus::Maintenance),
            _ => None,
        }
    }
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Unhealthy => "unhealthy",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "healthy" => Some(HealthStatus::Healthy),
            "degraded" => Some(HealthStatus::Degraded),
            "unhealthy" => Some(HealthStatus::Unhealthy),
            _ => None,
        }
    }
}

/// Provider capability
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProviderCapability {
    Speech,
    Transcription,
    Translation,
    SoundEffect,
    Music,
}

impl ProviderCapability {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderCapability::Speech => "speech",
            ProviderCapability::Transcription => "transcription",
            ProviderCapability::Translation => "translation",
            ProviderCapability::SoundEffect => "sound_effect",
            ProviderCapability::Music => "music",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "speech" => Some(ProviderCapability::Speech),
            "transcription" => Some(ProviderCapability::Transcription),
            "translation" => Some(ProviderCapability::Translation),
            "sound_effect" => Some(ProviderCapability::SoundEffect),
            "music" => Some(ProviderCapability::Music),
            _ => None,
        }
    }
}

/// Audio provider route entity
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AudioProviderRoute {
    pub id: i64,
    pub route_key: String,
    pub route_name: String,
    pub provider_id: String,
    pub provider_type: String,
    pub client_protocol: String,
    pub upstream_protocol: String,
    pub upstream_config_json: String,
    pub capabilities_json: String,
    pub enabled: bool,
    pub health_status: String,
    pub managed_by: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub version: i64,
}

/// Create provider route request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderRouteRequest {
    pub route_key: String,
    pub route_name: String,
    pub provider_id: String,
    pub provider_type: ProviderType,
    pub client_protocol: String,
    pub upstream_protocol: String,
    pub upstream_config_json: String,
    pub capabilities: Vec<ProviderCapability>,
    pub enabled: bool,
    pub managed_by: String,
    pub notes: Option<String>,
}

/// Update provider route request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderRouteRequest {
    pub route_name: Option<String>,
    pub provider_type: Option<ProviderType>,
    pub upstream_config_json: Option<String>,
    pub capabilities: Option<Vec<ProviderCapability>>,
    pub enabled: Option<bool>,
    pub health_status: Option<HealthStatus>,
    pub notes: Option<String>,
}

/// Provider route filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRouteFilter {
    pub provider_type: Option<ProviderType>,
    pub capability: Option<ProviderCapability>,
    pub enabled: Option<bool>,
    pub health_status: Option<HealthStatus>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
}

/// Provider route list result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRouteListResult {
    pub routes: Vec<AudioProviderRoute>,
    pub total: i64,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}
