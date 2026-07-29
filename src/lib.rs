//! # Insion Content Moderation API SDK
//!
//! The official Rust SDK for the Insion Content Moderation API.
//!
//! ## Getting Started
//!
//! ```rust
//! use insion::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         token: Some("<token>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = InsionClient::new(config).expect("Failed to build client");
//!     client
//!         .moderate_a_record(
//!             &ModerateRequest {
//!                 client_id: "clientId".to_string(),
//!                 name: "name".to_string(),
//!                 entity: "entity".to_string(),
//!                 content: Content::String("content".to_string()),
//!                 client_url: None,
//!                 metadata: None,
//!                 user: None,
//!                 passthrough: None,
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
