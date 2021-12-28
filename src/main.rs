use prime_listener::PrimeListener;
use structopt::StructOpt;
use std::sync::{Arc, Mutex};

mod opt;
mod server;
mod client;
mod twitch;
mod event_bus;
mod event;
mod prime_listener;
mod twitch_chat_listener;
mod quirk;

use opt::PiOpts;
use server::server;
use client::Client;
use event_bus::{Dispatcher, Dispatchable, run_dispatcher};
use event::Event;
use quirk::{Quirk, get_quirk_token};

use crate::twitch_chat_listener::TwitchChatListener;

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

        let mut events = Dispatcher::new();
        let twitch = twitch::Twitch::new(tx.clone()).await;
        let prime_events = Arc::new(Mutex::new(PrimeListener::new(tx.clone())));
        let twitch_chat_listener = Arc::new(Mutex::new(TwitchChatListener::new(tx.clone())));
        let mut client = Client::new();
        let quirk_token = get_quirk_token().await?;
        let quirk = Quirk::new(tx.clone(), quirk_token);

        client.connect(opt)?;

        events.register_listener(prime_events);
        events.register_listener(twitch_chat_listener);

        let client = Arc::new(Mutex::new(client));
        events.register_listener(client);

        tx.send(Event::StartOfProgram)?;

        println!("Running dispatcher");
        run_dispatcher(rx, events).await?;
        twitch.join_handle.await?;
        quirk.join_handle.await?;
    }


    return Ok(());
}
