use std::env;
use std::iter::zip;
use std::borrow::Cow;

use dotenvy::dotenv;

use serenity::async_trait;
use serenity::http::Http;
use serenity::model::channel::{AttachmentType, Message};
use serenity::model::prelude::Ready;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.webhook_id.is_some() || (msg.content.is_empty() && msg.attachments.is_empty()) {
            return;
        }

        let uri = if msg.channel_id.to_string() == env::var("CHANNEL_ONE_ID").expect("Expected CHANNEL_ONE_ID in the environment") {
            env::var("CHANNEL_TWO_HOOK").expect("Expected CHANNEL_TWO_HOOK in the environment")
        } else if msg.channel_id.to_string() == env::var("CHANNEL_TWO_ID").expect("Expected CHANNEL_TWO_ID in the environment") {
            env::var("CHANNEL_ONE_HOOK").expect("Expected CHANNEL_ONE_HOOK in the environment")
        } else {
            return;
        };

        let http = Http::new("");
        let webhook = Webhook::from_url(&http, &uri).await.expect("Replace the webhook with your own");

        let mut files = Vec::new();
        let mut filenames = Vec::new();
        for attachment in &msg.attachments {
            files.push(attachment.download().await.unwrap());
            filenames.push(attachment.filename.to_string());
        }

        webhook
            .execute(&http, false, |w| {
                w.content(msg.content_safe(&ctx))
                    .username(&msg.author.name)
                    .avatar_url(&msg.author.avatar_url().unwrap_or_default());

                for (file, filename) in zip(files, filenames) {
                    w.add_file(AttachmentType::Bytes {
                        data: Cow::from(file),
                        filename
                    });
                }

                w
            })
            .await
            .expect("Could not execute webhook.");
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

    let mut client =
        Client::builder(&token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
