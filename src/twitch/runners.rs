use futures_channel::mpsc::UnboundedSender;
use crate::event::{Event, TwitchChat};

pub trait Runner {
    fn matches(&mut self, event: &Event) -> bool;
}

fn is_prime(event: &TwitchChat) -> bool {
    return event.name.eq_ignore_ascii_case("theprimeagen");
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
        if let Event::TwitchChat(e) = event {
            if e.text.starts_with("!debug") {
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
        if let Event::TwitchChat(e) = event {
            if is_prime(e) && e.text.starts_with("!play") {

                if e.text == "!play stop" {
                    self.tx.unbounded_send(Event::Stop).expect("Successful tx send");
                } else {
                    self.tx.unbounded_send(Event::Play(e.text[5..].to_string())).expect("Successful tx send");
                }
                return true;
            }
        }
        return false;
    }
}

impl Runner for TurnMeDaddy {
    fn matches(&mut self, event: &Event) -> bool {
        if let Event::TwitchChat(e) = event {
            if is_prime(e) {
                match e.text.as_str() {
                    "turn_me_on_daddy" => {
                        self.tx.unbounded_send(Event::OnCommand).expect("prime commands shouldn't fail");
                        return true;
                    },
                    "turn_me_off_daddy" => {
                        self.tx.unbounded_send(Event::OffCommand).expect("prime commands shouldn't fail");
                        return true;
                    },
                    _ => { }
                }
            }
        }

        return false;
    }
}
