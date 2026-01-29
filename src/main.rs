use dotenvy::dotenv;
use min_rs_config::ConfigParser;
use std::env;
use uuid::Uuid;

use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

mod telegram_provider;
use min_rs::provider::{Data, Provider as MaxProvider};
use telegram_provider::*;

use crate::update::{UpdateConfig, Updater};

mod update;

pub type AsyncError = dyn std::error::Error + Send + Sync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::task::spawn_blocking(|| {
        Updater::new(UpdateConfig::AutoUpdate).update()
    }).await??;

    dotenv().ok();

    let config = ConfigParser::parse_config_file("config.json")?;

    let token = env::var("TOKEN").expect("Token is required in .env file!");

    let (tx, mut rx) = mpsc::channel::<String>(1024);

    let user_agent_data = Data {
        // device_id: Some("13977301-4cfd-4cb4-98b6-3536e0744015".to_string()),
        device_id: Some(Uuid::new_v4().to_string()),
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

    let mut provider =
        MaxProvider::new(
            serde_json::to_string(&config.headers)?,
            "wss://ws-api.oneme.ru/websocket".to_string(),
            tx,
            user_agent_data,
            auth_data,
        )
        .await?;

    let new_data = Data {
        backward: Some(30),
        chat_id: Some(114034918),
        forward: Some(0),
        from: Some(1766430956433),
        get_messages: Some(true),
        ..Default::default()
    };

    // let max_provider_clone = Arc::clone(&provider);
    let handle = tokio::spawn(async move {
        provider.auth().await.unwrap();

        // provider.send_data(new_data, 49).await.unwrap();

        provider.handle_everything().await.unwrap();
    });

    let telegram_provider = Arc::new(Mutex::new(TelegramProvider::new()?));
    let telegram_bridge_clone = Arc::clone(&telegram_provider);
    let telegram_bridge_handle = tokio::spawn(async move {
        println!("Waiting for messages");
        while let Some(data) = rx.recv().await {
            match telegram_bridge_clone
                .lock()
                .await
                .send_message("1021952704".to_string(), data.to_string())
                .await
            {
                Ok(_) => {/*println!("Message has been sent to telegram")*/},
                Err(e) => println!("Error sending message to telegram: {}", e),
            };
        }
    });
    let telegram_provider_clone = Arc::clone(&telegram_provider);
    telegram_provider_clone.lock().await.handle_messages().await;
    println!(
        "{:#?}",
        telegram_provider.lock().await.get_my_username().await?
    );

    handle.await?;
    // inters_handle.await?;
    telegram_bridge_handle.await?;

    Ok(())
}
