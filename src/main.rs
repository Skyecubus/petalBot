use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use regex::Regex;

struct Bot;

struct MessageCounter;

impl TypeMapKey for MessageCounter{
    type Value = i32;
}

//increments the counter
async fn countUP(ctx: &Context){
    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<MessageCounter>().unwrap();
    *counter += 1;
    //debug remove later 
    println!("{}", counter);

}

//checks if counter = 10
async fn checkCount(ctx: &Context) -> bool {
    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<MessageCounter>().unwrap();

    if *counter >= 3{
        *counter = 0;
        true
    }else{
        false
    }
}


//some call words are: 
//good floret, !ping, !hello, !gay, !yay!, !trans rights, !trans wrongs
#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.author.bot == false{

            countUP(&ctx).await;

            if checkCount(&ctx).await{
            self.respond(ctx, msg, "50!").await;
            } 

            else if Regex::new(r".*who(?:mst)?.*petal *bot.*|.*petal *bot.*who(?:mst)?.*").expect("reason").captures(&msg.content.to_lowercase())
                .is_some(){
                    self.respond(ctx, msg, "My name is mira diocelia, First Floret! my mistress is actenosa diocelia, 7th bloom!").await;
                }
        

            else if Regex::new(r".*good floret.*").expect("reason").captures(&msg.content.to_lowercase()).is_some() {
                self.respond(ctx, msg, "get florted bitch").await;
            }

            //oh no
            else if Regex::new(r".*o+h+ n+o+.**").expect("reason").captures(&msg.content.to_lowercase()).is_some() {
                self.respond(ctx, msg, "oh no! mistress we need xenodruggies stat!").await;
            }

            else if Regex::new(r".*good floret.*").expect("reason").captures(&msg.content.to_lowercase()).is_some() {
                self.respond(ctx, msg, "get florted bitch").await;
            }

            else if msg.content.to_lowercase() == "!ping"{
                println!("Shard {}", ctx.shard_id);

                self.respond(ctx, msg, "pong!").await;
            }

            
            else if msg.content.to_lowercase() == "!hello"{
                self.respond(ctx, msg, "world!").await;
            }
            

            else if msg.content.to_lowercase() == "!gay" {
                self.respond(ctx, msg, "Im the gayest little girl!").await;
            }


            else if Regex::new(r".*yay!.*").expect("reason").captures(&msg.content.to_lowercase()).is_some() {
                self.respond(ctx, msg, "Yaaaaaaaaaaay,,,,,,,,!!!!!!🏳️‍⚧️").await;
            }


            else if msg.content.to_lowercase() == "!trans rights"{
                self.respond(ctx, msg, "Trans Rights!!! 🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️").await;
            }

            else if msg.content.to_lowercase() == "!trans wrongs"{
                self.respond(ctx, msg, "Trans Wrongs!!! 💙🩷🤍🩷💙").await;
            }

        }

       


    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}


impl Bot{
    async fn respond(&self, ctx: Context, msg: Message, response: &str){
        if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
            error!("Error sending message: {:?}", e);
        }
        
    }

}


#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<MessageCounter>(0);
    }

    if let Err(e) = client.start_shards(2). await {
        println!("Client error: {e:?}");
    }


    Ok(client.into())
}


