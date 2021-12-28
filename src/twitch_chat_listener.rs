use tokio::sync::mpsc::UnboundedSender;
use twitch_irc::message::ServerMessage;

use crate::{event::Event, event_bus::Listener};

pub struct TwitchChatListener {
    tx: UnboundedSender<Event>,
}

/*
*/

impl Listener for TwitchChatListener {
    fn notify(&mut self, event: &Event) {
        if let Event::TwitchIRC(ServerMessage::Privmsg(e)) = event {
            if [
                "drum_heavy_kick",
                "drum_tom_mid_soft",
                "drum_tom_mid_hard",
                "drum_tom_lo_soft",
                "drum_tom_lo_hard",
                "drum_tom_hi_soft",
                "drum_tom_hi_hard",
                "drum_splash_soft",
                "drum_splash_hard",
                "drum_snare_soft",
                "drum_snare_hard",
                "drum_cymbal_soft",
                "drum_cymbal_hard",
                "drum_cymbal_open",
                "drum_cymbal_closed",
                "drum_cymbal_pedal",
                "drum_bass_soft",
                "drum_bass_hard",
            ].iter().filter(|t| ***t == e.message_text).count() > 0 {
                println!("Sending command: {}", e.message_text);
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

