use std::env;

use dotenvy::dotenv;

use serenity::async_trait;
use serenity::http::Http;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.webhook_id.is_some() {
            return;
        }

        let uri;
        if msg.channel_id.to_string() == env::var("CHANNEL_ONE_ID").unwrap() {
            uri = env::var("CHANNEL_TWO_HOOK").unwrap();
        } else if msg.channel_id.to_string() == env::var("CHANNEL_TWO_ID").unwrap() {
            uri = env::var("CHANNEL_ONE_HOOK").unwrap();
        } else {
            return;
        }

        let http = Http::new("");
        let webhook = Webhook::from_url(&http, &*uri).await.expect("Replace the webhook with your own");

        webhook
            .execute(&http, false, |w|
                w.content(msg.content_safe(&ctx))
                    .username(&msg.author.name)
                    .avatar_url(&msg.author.avatar_url().unwrap_or("".to_string()))
            )
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

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
