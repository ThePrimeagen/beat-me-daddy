use tokio::sync::mpsc::UnboundedSender;
use twitch_irc::message::ServerMessage;

use crate::{event::Event, event_bus::Listener};

pub struct TwitchChatListener {
    tx: UnboundedSender<Event>,
}

/*
*/

pub fn allow(_nick: &str) -> bool {
    /*
    return [
        "oldmanjudo",
        "ThePrimeagen",
        "theprimeagen",
    ].contains(&nick.as_str());
    */
    return true;
}

impl Listener for TwitchChatListener {
    fn notify(&mut self, event: &Event) {
        if let Event::TwitchIRC(ServerMessage::Privmsg(e)) = event {
            if allow(&e.sender.name) && crate::bangers::boolizer::is_bang_command(&e.message_text) {
                self.tx.send(Event::DrumCommand(e.message_text.clone())).expect("Successful successing of drum successions");
            }
        } else if let Event::QuirkMessage(s) = event {
            println!("Message from Quirk {}", s);
        }
    }
}

impl TwitchChatListener {
    pub fn new(tx: UnboundedSender<Event>) -> TwitchChatListener {
        return TwitchChatListener {
            tx
        };
    }
}

