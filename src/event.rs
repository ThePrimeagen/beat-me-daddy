use std::thread::{self, JoinHandle};

use log::info;
use serde::{Deserialize};
use futures_channel::mpsc::UnboundedSender;
use tungstenite::connect;
use url::Url;

#[derive(Debug, Clone)]
pub enum Event {
    EventsMessage(String),
    TwitchChat(TwitchChat),
    TwitchEvent(TwitchEvent),
    QuirkMessage(String),
    DrumCommand(String),
    Stop,
    Play(String),
    OnCommand,
    OffCommand,
    StartOfProgram,
}

pub struct Eventer {
    pub join_handle: JoinHandle<()>,
}

#[derive(Debug, Clone, Deserialize)]
struct PartialDecode {
    source: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwitchChat {
    pub text: String,
    pub name: String,
    pub sub: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwitchEventDataReward {
    pub title: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TwitchEventData {
    pub user_name: String,
    pub reward: TwitchEventDataReward,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwitchEvent {
    pub data: TwitchEventData
}

fn process_msg(text: String) -> Result<Option<Event>, Box<dyn std::error::Error>> {
    if !text.contains("source\":") {
        return Ok(None);
    }
    let partial = serde_json::from_str::<PartialDecode>(&text.clone()).expect("if it has source, then it should be fine");

    return match partial.source.as_str() {
        "TWITCH_CHAT" => {
            info!("TwitchChat received");
            Ok(Some(Event::TwitchChat(serde_json::from_str::<TwitchChat>(&text)?)))
        }
        "TWITCH_EVENTSUB" => {
            info!("TwitchEventSub received");
            Ok(Some(Event::TwitchEvent(serde_json::from_str::<TwitchEvent>(&text)?)))
        }
        _ => Ok(None),
    }
}

impl Eventer {
    pub fn new(tx: UnboundedSender<Event>) -> Eventer {
        println!("eventer::new");
        let (mut socket, _) =
            connect(Url::parse("ws://events.theprimeagen.tv:42069").unwrap())
                .expect("Can't connect to the events.theprimeagen.tv");
        println!("eventer::connected");

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle: JoinHandle<()> = thread::spawn(move || {
            while let Ok(msg) = socket.read_message() {
                println!("reading messages for event");
                if msg.is_text() {
                    let text = msg.into_text().expect("A text messaget should have text");
                    println!("TEXT: {}", text);
                    if let Ok(Some(event)) = process_msg(text) {

                        println!("sending from eventer");
                        tx.unbounded_send(event).expect("send to be successful?");
                        println!("sent from eventer");
                    }
                }
            }
            return ();
        });

        return Eventer {
            join_handle,
        };
    }
}

