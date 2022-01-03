

use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use twitch_irc::login::{StaticLoginCredentials};
use twitch_irc::TwitchIRCClient;

use twitch_irc::{ClientConfig, SecureTCPTransport};

use crate::event::Event;

pub struct Twitch {
    pub join_handle: Option<JoinHandle<()>>,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>
}

impl Twitch {
    pub async fn send_message(&mut self, s: String) -> Result<(), Box<dyn std::error::Error>> {

        self.client.privmsg("theprimeagen".to_string(), s).await?;

        return Ok(());
    }

    pub async fn new(tx: Option<UnboundedSender<Event>>) -> Twitch {

        let login_name = std::env::var("OAUTH_NAME").expect("OAUTH_NAME is required for twitch client");
        let oauth_token = std::env::var("OAUTH_TOKEN").expect("OAUTH_TOKEN is required for twitch client");

        let config: ClientConfig<StaticLoginCredentials> = ClientConfig::new_simple(StaticLoginCredentials::new(
            login_name,
            Some(oauth_token),
        ));

        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // join a channel
        client.join("theprimeagen".to_owned());

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle: Option<JoinHandle<()>>;

        if let Some(tx) = tx {
            join_handle = Some(tokio::spawn(async move {
                loop {
                    if let Some(message) = incoming_messages.recv().await {
                        tx.send(Event::TwitchIRC(message)).expect("Never going to give you up");
                    } else {
                        print!("LOOK AT ME FAIL");
                    }
                }
            }));
        } else {
            join_handle = None;
        }

        return Twitch {
            join_handle,
            client,
        };
    }
}

