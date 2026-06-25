//! SDKWork Audio speech synthesis service
//!
//! This crate provides the speech synthesis service implementation.

pub mod service;
pub mod models;
pub mod error;

pub use service::*;
pub use models::*;
pub use error::*;
