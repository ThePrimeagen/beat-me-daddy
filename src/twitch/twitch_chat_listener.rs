use futures_channel::mpsc::UnboundedSender;

use crate::{event::Event, event_bus::Listener};

pub struct TwitchChatListener {
    tx: UnboundedSender<Event>,
}

/*
*/

pub fn allow(_nick: &String) -> bool {
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
        println!("TwitchChatListener#notify: {:?}", event);
        if let Event::TwitchChat(e) = event {
            println!("is twitch_chat");
            if allow(&e.name) && crate::bangers::boolizer::is_bang_command(&e.text) {
                self.tx.unbounded_send(Event::DrumCommand(e.text.clone())).expect("Successful successing of drum successions");
            }
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

