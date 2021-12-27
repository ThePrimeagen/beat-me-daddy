use tokio::sync::mpsc::UnboundedSender;
use twitch_irc::message::ServerMessage;

use crate::{event::Event, event_bus::Listener};

pub struct PrimeListener {
    tx: UnboundedSender<Event>
}

impl Listener for PrimeListener {
    fn notify(&mut self, event: &Event) {
        if let Event::TwitchIRC(ServerMessage::Privmsg(e)) = event {
            if e.sender.name.eq_ignore_ascii_case("theprimeagen") {
                match e.message_text.as_str() {
                    "turn_me_on_daddy" => {
                        self.tx.send(Event::OnCommand).expect("prime commands shouldn't fail");
                    },
                    "turn_me_off_daddy" => {
                        self.tx.send(Event::OffCommand).expect("prime commands shouldn't fail");
                    },
                    _ => {}
                }
            }
        }
    }
}

impl PrimeListener {
    pub fn new(tx: UnboundedSender<Event>) -> PrimeListener {
        return PrimeListener {
            tx
        };
    }
}
