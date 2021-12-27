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

use opt::PiOpts;
use server::server;
use client::client;
use event_bus::{Dispatcher, Dispatchable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Arc::new(PiOpts::from_args());
    let events = Arc::new(Mutex::new(Dispatcher::new()));
    let twitch = twitch::Twitch::new(events.clone()).await;

    let prime_events = Arc::new(Mutex::new(PrimeListener::new()));
    events.lock().expect("events lock cannot fail here").register_listener(prime_events);

    if opt.debug {
        println!("{:?}", opt);
    }

    if opt.server {
        server(opt)?;
    } else {
        client(opt)?;
    }

    twitch.join_handle.await?;

    return Ok(());
}
