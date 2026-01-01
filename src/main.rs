use dotenvy::dotenv;
use std::env;

use tokio::sync::Mutex;
use std::sync::Arc;

mod max_provider;
mod config;
use config::*;
use max_provider::{ Provider as MaxProvider, Data };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = ConfigParser::parse_config_file("config.json")?;

    let token = env::var("TOKEN").expect("Token is required in .env file!");

    let provider = Arc::new(Mutex::new(MaxProvider::new(serde_json::to_string(&config.headers)?, "wss://ws-api.oneme.ru/websocket".to_string()).await?));

    let user_agent_data = Data {
        device_id: Some("13977301-4cfd-4cb4-98b6-3536e0744015".to_string()),
        user_agent: Some(config.max_agent),
        ..Default::default()
    };

    let auth_data = Data {
        chats_count: Some(40),
        chats_sync: Some(0),
        contacts_sync: Some(0),
        drafts_sync: Some(0),
        interactive: Some(true),
        presence_sync: Some(-1),
        token: Some(token.to_string()),
        ..Default::default()
    };

    let max_provider_clone = Arc::clone(&provider);
    let handle = tokio::spawn(async move {
        let mut guard = max_provider_clone.lock().await;
        guard.send_data(user_agent_data, 6).await.unwrap();
        guard.send_data(auth_data, 19).await.unwrap();
        guard.handle_messages().await.unwrap();
    });

    provider.lock().await.send_data(Data {
        ..Default::default()
    }, 17).await?;

    for i in 0..=10 {
        println!("Hello world {}!", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    handle.await?;

    Ok(())
}
