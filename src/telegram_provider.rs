use teloxide::prelude::*;
use teloxide::types::User;
use std::io::Error;
use std::env;

async fn process_text_message(bot: Bot, user: User, message_text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Answering to user {}: {}", user.id, message_text);
    bot.send_message(user.id, format!("Hi! Your telegram id is: {}", user.id)).await?;
    Ok(())
}

pub struct TelegramProvider {
    pub bot: Bot,
}

impl TelegramProvider {
    pub fn new() -> Result<Self, Error> {
        let bot = Bot::new(env::var("T_TOKEN").unwrap());

        Ok(Self {
            bot
        })
    }

    pub async fn get_my_username(&self) -> Result<String, Error> {
        Ok(self.bot.get_me().await.unwrap().user.first_name)
    }

    pub async fn handle_messages(&self) {
        let bot_clone = self.bot.clone();
        tokio::spawn(async move {
            let schema = Update::filter_message()
            .filter_map(|update: Update| update.from().cloned())
            .branch(
                Message::filter_text().endpoint(process_text_message),
            );

            Dispatcher::builder(bot_clone, schema).build().dispatch().await;
        });
    }

    pub async fn send_message(&self, chat_id: String, message_text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.bot.send_message(chat_id, message_text).await?;
        Ok(())
    }
}