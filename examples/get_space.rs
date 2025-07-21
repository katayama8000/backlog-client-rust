use backlog_client::{BacklogClient, Result};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // .env ファイルから環境変数を読み込み
    dotenv().ok();

    // 環境変数から設定を読み取り
    let base_url = env::var("BACKLOG_BASE_URL")
        .unwrap_or_else(|_| "https://yourspace.backlog.com".to_string());

    let api_key = env::var("BACKLOG_API_KEY")
        .expect("BACKLOG_API_KEY environment variable must be set in .env file");

    println!("Connecting to: {}", base_url);

    // Initialize the client with your Backlog space URL and API key
    let client = BacklogClient::new(&base_url, &api_key);

    // Get space information
    println!("=== Getting space information ===");
    match client.get_space().await {
        Ok(space) => {
            println!(
                "space key: {} space name: {} owner id: {} language: {} timezone: {}",
                space.space_key, space.name, space.owner_id, space.lang, space.timezone
            );
        }
        Err(e) => println!("Error getting space: {}", e),
    }

    Ok(())
}
