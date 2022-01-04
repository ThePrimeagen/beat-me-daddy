#![allow(clippy::needless_return)]
use beatmedaddy::event_bus::{Dispatchable, Dispatcher, run_dispatcher};
use structopt::StructOpt;
use std::sync::{Arc, Mutex};

mod opt;
mod server;
mod client;
mod quirk;
mod bangers;

use opt::PiOpts;
use server::server;
use client::Client;

use beatmedaddy::event::Event;
use beatmedaddy::twitch::{
    prime_listener::PrimeListener,
    twitch_client::Twitch,
    twitch_chat_listener::TwitchChatListener,
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let opt = Arc::new(PiOpts::from_args());

    if opt.debug {
        println!("{:?}", opt);
    }

    if opt.server {
        server(opt).expect("The server should never fail");
    } else {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let mut events = Dispatcher::default();
        let twitch = Twitch::new(Some(tx.clone())).await;
        let prime_events = Arc::new(Mutex::new(PrimeListener::new(tx.clone())));
        let twitch_chat_listener = Arc::new(Mutex::new(TwitchChatListener::new(tx.clone())));
        let mut client = Client::new();

        client.connect(opt)?;

        events.register_listener(prime_events);
        events.register_listener(twitch_chat_listener);

        let client = Arc::new(Mutex::new(client));
        events.register_listener(client);

        tx.send(Event::StartOfProgram)?;

        println!("Running dispatcher");
        run_dispatcher(rx, events).await?;
        if let Some(join_handle) = twitch.join_handle {
            join_handle.await?;
        }

    }


    return Ok(());
}
