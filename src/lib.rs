mod client;
mod types;

pub use client::{BacklogClient, BacklogError, BacklogResult};
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
        let client = BacklogClient::with_client(
            "https://test.backlog.com",
            "test_key",
            reqwest::Client::new(),
        );
        // Test that custom client creation works
        drop(client);
    }
}
