use std::sync::{Arc, Mutex};

use tokio::task::JoinHandle;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use crate::event::Event;
use crate::event_bus::Dispatcher;

pub struct Twitch {
    pub join_handle: JoinHandle<()>,
}

impl Twitch {
    pub async fn new(events: Arc<Mutex<Dispatcher>>) -> Twitch {
        let config: ClientConfig<StaticLoginCredentials> = ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle: JoinHandle<()> = tokio::spawn(async move {

            // join a channel
            client.join("theprimeagen".to_owned());

            while let Some(message) = incoming_messages.recv().await {
                match events.lock() {
                    Ok(mut e) => {
                        e.dispatch(Event::TwitchIRC(message)).expect("Should always successfully dispatch twitch messages");
                    },
                    _ => {}
                }
            }

            return ();
        });

        return Twitch {
            join_handle,
        };
    }
}

/*
#[tokio::main]
pub async fn main() {
    // default configuration is to join chat as anonymous.

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}

*/
