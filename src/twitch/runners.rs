use tokio::sync::mpsc::UnboundedSender;
use twitch_irc::message::{ServerMessage, PrivmsgMessage};
use crate::event::Event;

pub trait Runner {
    fn matches(&mut self, event: &Event) -> bool;
}

fn is_prime(event: &PrivmsgMessage) -> bool {
    return event.sender.name.eq_ignore_ascii_case("theprimeagen");
}

pub struct TurnMeDaddy {
    pub tx: UnboundedSender<Event>,
}

pub struct PlayTheThing {
    pub tx: UnboundedSender<Event>,
}

pub struct Debug { }
impl Runner for Debug {
    fn matches(&mut self, event: &Event) -> bool {
        if let Event::TwitchIRC(ServerMessage::Privmsg(e)) = event {
            if e.message_text.starts_with("!debug") {
                let test = '♥';
                /*
                print!("DEBUG: ");
                e.message_text.chars().filter(|c| (*c as u32 >= test as u32)).for_each(|c| {
                    print!("{} ", (c as u32 - test as u32));
                });
                */
                for t in 1..1024 {
                    if t % 500 == 0 {
                        println!();
                        println!("----");
                    }
                    print!("{} ", char::from_u32(test as u32 + t).unwrap());
                }
                println!();
                return true;
            }
        }
        return true;
    }
}

impl Runner for PlayTheThing {
    fn matches(&mut self, event: &Event) -> bool {
        if let Event::TwitchIRC(ServerMessage::Privmsg(e)) = event {
            if is_prime(e) && e.message_text.starts_with("!play") {

                if e.message_text == "!play stop" {
                    self.tx.send(Event::Stop).expect("Successful tx send");
                } else {
                    self.tx.send(Event::Play(e.message_text[5..].to_string())).expect("Successful tx send");
                }
                return true;
            }
        }
        return false;
    }
}

impl Runner for TurnMeDaddy {
    fn matches(&mut self, event: &Event) -> bool {
        if let Event::TwitchIRC(ServerMessage::Privmsg(e)) = event {
            if is_prime(e) {
                match e.message_text.as_str() {
                    "turn_me_on_daddy" => {
                        self.tx.send(Event::OnCommand).expect("prime commands shouldn't fail");
                        return true;
                    },
                    "turn_me_off_daddy" => {
                        self.tx.send(Event::OffCommand).expect("prime commands shouldn't fail");
                        return true;
                    },
                    _ => { }
                }
            }
        }

        return false;
    }
}
