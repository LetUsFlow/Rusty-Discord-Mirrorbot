use std::env;
use std::iter::zip;
use std::borrow::Cow;
use std::sync::Arc;

use dotenvy::dotenv;

use serenity::async_trait;
use serenity::http::Http;
use serenity::json::json;
use serenity::model::channel::{AttachmentType, Message};
use serenity::model::prelude::Ready;
use serenity::model::webhook::Webhook;
use serenity::prelude::*;

struct Handler {
    http: Arc<Http>,
    wh_one: Webhook,
    wh_two: Webhook,
    id_one: String,
    id_two: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Only ignore messages from the bots own webhooks
        if msg.webhook_id.unwrap_or_default() == self.wh_one.id ||
            msg.webhook_id.unwrap_or_default() == self.wh_two.id {
            return;
        }

        // Pick correct webhook
        let webhook = if msg.channel_id.to_string() == self.id_one {
            &self.wh_two
        } else if msg.channel_id.to_string() == self.id_two {
            &self.wh_one
        } else {
            return;
        };

        // Extract attachments
        let mut files = Vec::new();
        let mut filenames = Vec::new();
        for attachment in &msg.attachments {
            files.push(attachment.download().await.unwrap());
            filenames.push(attachment.filename.to_string());
        }

        webhook
            .execute(&self.http, false, |w| {
                w.content(msg.content_safe(&ctx))
                    .username(&msg.author.name)
                    .avatar_url(&msg.author.avatar_url().unwrap_or_default());

                for (file, filename) in zip(files, filenames) {
                    w.add_file(AttachmentType::Bytes {
                        data: Cow::from(file),
                        filename,
                    });
                }

                if !msg.embeds.is_empty() {
                    w.embeds(msg.embeds.iter().map(|e| json!(e)).collect());
                }

                w
            })
            .await
            .expect("Could not execute webhook");
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

    let http = Arc::new(Http::new(""));

    let handler = Handler {
        http: http.clone(),
        wh_one: Webhook::from_url(&http, &env::var("CHANNEL_ONE_HOOK").expect("Expected CHANNEL_ONE_HOOK in the environment"))
            .await.expect("Creating CHANNEL_ONE_HOOK failed"),
        wh_two: Webhook::from_url(&http, &env::var("CHANNEL_TWO_HOOK").expect("Expected CHANNEL_TWO_HOOK in the environment"))
            .await.expect("Creating CHANNEL_TWO_HOOK failed"),
        id_one: env::var("CHANNEL_ONE_ID").expect("Expected CHANNEL_ONE_ID in the environment"),
        id_two: env::var("CHANNEL_TWO_ID").expect("Expected CHANNEL_TWO_ID in the environment"),
    };

    let mut client =
        Client::builder(&token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT).event_handler(handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
