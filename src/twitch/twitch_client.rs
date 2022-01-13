



use twitch_irc::login::{StaticLoginCredentials};
use twitch_irc::TwitchIRCClient;

use twitch_irc::{ClientConfig, SecureTCPTransport};

pub struct Twitch {
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>
}

impl Twitch {
    pub async fn send_message(&mut self, s: String) -> Result<(), Box<dyn std::error::Error>> {

        self.client.privmsg("theprimeagen".to_string(), s).await?;

        return Ok(());
    }

    pub fn new() -> Twitch {

        let login_name = std::env::var("OAUTH_NAME").expect("OAUTH_NAME is required for twitch client");
        let oauth_token = std::env::var("OAUTH_TOKEN").expect("OAUTH_TOKEN is required for twitch client");

        let config: ClientConfig<StaticLoginCredentials> = ClientConfig::new_simple(StaticLoginCredentials::new(
            login_name,
            Some(oauth_token),
        ));

        let (_, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // join a channel
        client.join("theprimeagen".to_owned());

        return Twitch {
            client,
        };
    }
}

