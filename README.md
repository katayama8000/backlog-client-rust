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

Currently implemented:
- ✅ Space API: `GET /api/v2/space` - Get space information

## Installation

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

### Using .env file

Create a `.env` file:
```
BACKLOG_BASE_URL=https://yourspace.backlog.com
BACKLOG_API_KEY=your_api_key_here
```

```rust
use backlog_client::{BacklogClient, Result};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let base_url = env::var("BACKLOG_BASE_URL").expect("BACKLOG_BASE_URL not set");
    let api_key = env::var("BACKLOG_API_KEY").expect("BACKLOG_API_KEY not set");
    
    let client = BacklogClient::new(&base_url, &api_key);
    let space = client.get_space().await?;
    println!("Space: {}", space.name);
    
    Ok(())
}
```

## Development

## Development

### Setup

1. Clone the repository:
```bash
git clone https://github.com/katayama8000/backlog-client-rust.git
cd backlog-client-rust
```

2. Create a `.env` file with your Backlog credentials:
```bash
cp .env.example .env
```

Edit `.env` file:
```
BACKLOG_BASE_URL=https://yourspace.backlog.com
BACKLOG_API_KEY=your_api_key_here
```

3. Build the project:
```bash
cargo build
```

### Testing

#### Basic Test
```bash
# Create .env file with your credentials
echo "BACKLOG_BASE_URL=https://yourspace.backlog.com" > .env
echo "BACKLOG_API_KEY=your_api_key_here" >> .env

# Run basic example
cargo run --example basic_usage
```

#### Debug Client
For debugging and testing the client:
```bash
# Simple debug test
cargo run --example debug_client

# Advanced debug with interactive mode
cargo run --example debug_advanced -- --interactive
```

#### Unit Tests
```bash
cargo test
```

## License

MIT License

## Contributing

Pull requests and issue reports are welcome.

## References

- [Backlog API Documentation](https://developer.nulab.com/docs/backlog/)
- [Backlog4j (Official Java Library)](https://github.com/nulab/backlog4j)
