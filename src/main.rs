#![feature(slice_group_by)]

use beatmedaddy::event_bus::{Dispatchable, Dispatcher, run_dispatcher};
use futures_channel::mpsc::unbounded;

use structopt::StructOpt;
use std::sync::{Arc, Mutex};

mod opt;
mod server;
mod client;

use opt::PiOpts;
use server::server;
use client::Client;

use beatmedaddy::event::{Event, Eventer};
use beatmedaddy::twitch::{
    prime_listener::PrimeListener,
    twitch_chat_listener::TwitchChatListener,
};

pub const STARTING_UTF: char = '♥';

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    env_logger::init();

    let opt = Arc::new(PiOpts::from_args());
    print!("drummers: ");

    for i in 22..1024 {
        print!("{}", char::from_u32(STARTING_UTF as u32 + i).unwrap());
    }
    println!();

    if opt.debug {
        println!("{:?}", opt);
    }

    if opt.server {
        server(opt).expect("The server should never fail");
    } else {
        let (tx, rx) = unbounded();

        let mut events = Dispatcher::new();
        let prime_events = Arc::new(Mutex::new(PrimeListener::new(tx.clone())));
        let twitch_chat_listener = Arc::new(Mutex::new(TwitchChatListener::new(tx.clone())));
        let mut client = Client::new(opt.clone());
        let eventer = Eventer::new(tx.clone());

        client.connect(opt)?;

        events.register_listener(prime_events);
        events.register_listener(twitch_chat_listener);

        let client = Arc::new(Mutex::new(client));
        events.register_listener(client);

        tx.unbounded_send(Event::StartOfProgram)?;

        println!("Running dispatcher");
        run_dispatcher(rx, events).await?;
        match eventer.join_handle.join() {
            Err(e) => panic!("eventer didn't join {:?}", e),
            _ => {}
        }
        println!("Game Over Asshole");
    }


    return Ok(());
}
