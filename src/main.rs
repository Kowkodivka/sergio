use std::env;
use std::sync::Arc;

use serenity::client::bridge::gateway::ShardManager;
use serenity::model::gateway::Ready;
use serenity::{async_trait, prelude::*};
use tracing::{error, info};

mod commands;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;
// commands: Mutex<HashMap<CommandId, &'static dyn Fn(&[CommandDataOption]) -> String>>,

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is ready", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // This will load env variables from `.env` file.
    dotenv::dotenv().expect("Failed to .env configuration");
    // Also we need to log messages to the console.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Cannot find DISCORD_TOKEN variable");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create discord client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    // We kill the shard manager when the client is dropped.
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to register ctrl+c signal handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    // Error will be returned if the shard manager fails to start.
    if let Err(e) = client.start().await {
        error!("Failed to login to discord: {:?}", e);
    }
}
