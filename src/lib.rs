//! # Backlog Client for Rust
//!
//! A Rust client library for the Backlog API with API key authentication support.
//!
//! ## Features
//!
//! - API key authentication
//! - Asynchronous requests using tokio and reqwest
//! - Type-safe JSON serialization/deserialization
//! - Comprehensive error handling
//! - Space API support
//!
//! ## Quick Start
//!
//! ```no_run
//! use backlog_client::{BacklogClient, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = BacklogClient::new("https://yourspace.backlog.com", "your_api_key");
//!     
//!     // Get space information
//!     let space = client.get_space().await?;
//!     println!("Space: {}", space.name);
//!     
//!     Ok(())
//! }
//! ```

mod client;
mod types;

pub use client::{BacklogClient, BacklogError, Result};
pub use types::*;

// Re-export commonly used types for convenience
pub use serde_json::Value as JsonValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BacklogClient::new("https://test.backlog.com", "test_api_key");
        // Just test that we can create a client without panicking
        // We'll test with a simple format check since fields are private
        let client2 = BacklogClient::new("https://example.backlog.com", "another_key");
        // If we got here without panicking, client creation works
        drop(client);
        drop(client2);
    }

    #[test]
    fn test_client_with_custom_client() {
        let http_client = reqwest::Client::new();
        let client =
            BacklogClient::with_client("https://test.backlog.com", "test_key", http_client);
        // Test that custom client creation works
        drop(client);
    }
}
