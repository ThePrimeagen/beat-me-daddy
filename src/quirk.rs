use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tungstenite::connect;
use url::Url;

use beatmedaddy::event::Event;

pub struct Quirk {
    pub join_handle: JoinHandle<()>,
}

#[derive(Deserialize, Serialize, Debug)]
struct RequestBody {
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseBody {
    access_token: String,
}

pub async fn get_quirk_token() -> Result<String, Box<dyn std::error::Error>> {
    let token = std::env::var("QUIRK_TOKEN").expect("QUIRK_TOKEN should be an env variable");
    let request = RequestBody { access_token: token };

    let client = reqwest::Client::new();
    let res: ResponseBody = client.post("https://websocket.quirk.tools/token")
        .json(&request)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await?;

    return Ok(res.access_token);
}

impl Quirk {
    pub fn new(tx: UnboundedSender<Event>, quirk_token: String) -> Quirk {
        let url = format!("wss://websocket.quirk.tools?access_token={}", quirk_token);
        let (mut socket, _) =
            connect(Url::parse(url.as_str()).unwrap())
                .expect("Can't connect");

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle: JoinHandle<()> = tokio::spawn(async move {
            while let Ok(msg) = socket.read_message() {
                if msg.is_text() {
                    let text = msg.into_text().expect("A text messaget should have text");
                    let event = Event::QuirkMessage(text);
                    tx.send(event).expect("send to be successful?");
                }
            }
            return ();
        });

        return Quirk {
            join_handle,
        };
    }
}

