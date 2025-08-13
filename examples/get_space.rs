use backlog_client::{BacklogClient, BacklogResult};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> BacklogResult<()> {
    dotenv().ok();
    let base_url = env::var("BACKLOG_BASE_URL")
        .expect("BACKLOG_BASE_URL environment variable must be set in .env file");
    let api_key = env::var("BACKLOG_API_KEY")
        .expect("BACKLOG_API_KEY environment variable must be set in .env file");

    println!("Connecting to: {}", base_url);

    let client = BacklogClient::new(&base_url, &api_key);

    println!("executing...");
    match client.get_space().await {
        Ok(space) => {
            println!(
                "space key: {} space name: {} owner id: {} language: {} timezone: {} text formatting rule: {:?}",
                space.space_key,
                space.name,
                space.owner_id,
                space.lang,
                space.timezone,
                space.text_formatting_rule
            );
        }
        Err(e) => println!("Error getting space: {:?}", e),
    }

    Ok(())
}
