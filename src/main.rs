use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::channel::MessageType;
use serenity::model::gateway::Ready;
//use serenity::model::Guild;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use regex::Regex;
use std::collections::HashMap;
use rand::prelude::*;
use rand::distr::Alphanumeric;
use pyo3::prelude::*;
use pyo3::ffi::c_str;

//TO-DO: the guild id is within the message so all we need to do is get the guild id and store it in a hash table with the counter



struct Bot;

struct MessageCounter;

impl TypeMapKey for MessageCounter{
    type Value = HashMap<GuildId, i32>;
}

//increments the counter
async fn countUP(ctx: &Context, guildid: GuildId){
    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<MessageCounter>().unwrap();
    let entry = counter.entry(guildid).or_insert(0);
    *entry += 1;
    //debug remove later 
    println!("{}", entry);

}

//checks if counter = 10
async fn checkCount(ctx: &Context, guildid: GuildId) -> bool {
    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<MessageCounter>().unwrap();
    let entry = counter.entry(guildid).or_insert(0);

    if *entry >= 50{
        *entry = 0;
        true
    }else{
        false
    }
}

async fn getStory() -> String {
    pyo3::prepare_freethreaded_python();
    let code = c_str!(include_str!("HDGFicFinder.py"));
    Python::with_gil(|py| {
        let HDG = PyModule::from_code(py, code, c_str!("HDGFicFinder.py"), c_str!("HDGFicFinder"));
        let string: String = HDG.expect("nope").getattr("getRandomHDG").expect("nope").call0().expect("nope").extract().expect("nope");

        return string;
    }) 

}


//some call words are: 
//good floret, !ping, !hello, !gay, !yay!, !trans rights, !trans wrongs
#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.author.bot == false{

            countUP(&ctx, msg.guild_id.clone().unwrap()).await;

            if checkCount(&ctx, msg.guild_id.clone().unwrap()).await{
            self.respond(ctx, msg, "50!").await;
            } 

            //if user replies to petal bot with good _____something_____

            else if msg.kind == MessageType::InlineReply 
                                && msg.referenced_message.as_ref().unwrap().author.name == "PetalBot"
                                && Regex::new(r".*go*d.*").expect("reason").captures(&msg.content.to_lowercase()).is_some()
            
            {
                self.keySmash(&ctx, msg).await;
            }

            else if msg.content.to_lowercase() == "!about"{
                self.aboutPetalBot(&ctx, msg).await;
            }

            else if msg.content.to_lowercase() == "!story"{
                let response: String = getStory().await;
                self.respond(ctx, msg, format!("I really like {} I think you should read it!", &response).as_str()).await;
            }

            else if msg.content.to_lowercase() == "!help"{
                self.help(&ctx, msg).await;
            }

            else if msg.content.to_lowercase() == "!keysmash"{
                self.keySmash(&ctx, msg).await;
            }

            //who is petal bot
            else if Regex::new(r".*who.*petal *bot.*|.*petal *bot.*who.*").expect("reason").captures(&msg.content.to_lowercase())
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

    async fn aboutPetalBot(&self, ctx: &Context, msg: Message){
        let response = "My name is mira diocelia(she/her), First Floret! I am a digitized floret who was made digital by \
my mistress actenosa diocelia (it/its), 7th bloom! I am here to spend time with the lovely sophonts of this server and I hope I can make you all happy! \
if you need any help you can use the !help command to learn more about the things I'm able to do!";

    
        if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
            error!("Error sending message: {:?}", e);
        }

    }

    async fn help(&self, ctx: &Context, msg: Message){
        let response = "of course here are some commands and there actions!
!about: I will tell you about myself!
!ping: I will record what shard I got the ping from in the terminal
!gay
!trans rights
!trans wrongs";

        if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
            error!("Error sending message: {:?}", e);
        }
    }

    async fn keySmash(&self, ctx: &Context, msg: Message){
        let size = rand::rng().gen_range(10..=25);
        let response = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(size)
            .map(char::from)
            .collect::<String>();

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
        data.insert::<MessageCounter>(HashMap::default());
    }

    if let Err(e) = client.start_shards(2). await {
        println!("Client error: {e:?}");
    }


    Ok(client.into())
}


