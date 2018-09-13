extern crate serenity;


extern crate reqwest;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate victoria_dom;

extern crate html2runes;
extern crate unicode_segmentation;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::env;
mod commands;

struct Handler;


impl EventHandler for Handler {


    fn message(&self, _: Context, msg: Message)  {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with("!termofis") {

            let term = str::replace(&msg.content, "!termofis ", "");
            // let message = commands::termofis_run(&term).unwrap();
            let message = commands::termofis_run(&term);

            if let Err(why) = msg.channel_id.say(message) {
                println!("Error sending message: {:?}", why);
            }
       }  else if msg.content.starts_with("!troer") {

           let term = str::replace(&msg.content, "!troer ", "");
           // let message = commands::termofis_run(&term).unwrap();
           let message = commands::troer_run(&term);

           if let Err(why) = msg.channel_id.say(message) {
               println!("Error sending message: {:?}", why);
           }
      } else if msg.content.starts_with("!difazi") {
            let term = str::replace(&msg.content, "!difazi ", "");

            let message = commands::languagetool_run(&term);

            msg.channel_id.say(&message);
       } else if msg.content.starts_with("!glosbe") {

            let term = str::replace(&msg.content, "!glosbe ", "");
            // let message = commands::glosbe_run(&term).unwrap();
            let message = commands::glosbe_run(&term);

            msg.channel_id.say(&message);
       }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
