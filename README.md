# backlog-client-rust

A Rust client library for the Backlog API

## Overview

This project provides a Rust client library for easily accessing the Nulab Backlog API.

## Features

- High-performance client written in Rust
- API Key authentication support
- Asynchronous processing with tokio
- Type-safe API responses with serde
- Comprehensive error handling
- Environment variable configuration support

## Authentication

### API Key Authentication

```rust
use backlog_client::{BacklogClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client with your Backlog space URL and API key
    let client = BacklogClient::new("https://yourspace.backlog.com", "your_api_key");
    
    // Get space information
    let space = client.get_space().await?;
    println!("Space: {}", space.name);
    
    Ok(())
}
```

### OAuth 2.0 Authentication
TODO

## Supported APIs

For a complete list of supported Backlog API endpoints, see [API.md](API.md).

## Usage Examples

### Basic Usage

```rust
use backlog_client::{BacklogClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = BacklogClient::new("https://yourspace.backlog.com", "your_api_key");
    
    let space = client.get_space().await?;
    println!("Space: {}", space.name);
    
    Ok(())
}
```
## License

MIT License

## Contributing

Pull requests and issue reports are welcome.

## References

- [Backlog API Documentation](https://developer.nulab.com/docs/backlog/)
- [Backlog4j (Official Java Library)](https://github.com/nulab/backlog4j)
