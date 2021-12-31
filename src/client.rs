use crate::{
    event::Event,
    event_bus::Listener,
    opt::PiOpts
};

use std::sync::Arc;
use beatmedaddy::bangers::{WriteNode, BangersSerializer, Direction};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

pub struct Client {
    on: bool,
    socket: Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>>,
    banger: beatmedaddy::bangers::Bangers,
}

impl Listener for Client {
    fn notify(&mut self, event: &Event) {
        match event {
            Event::OnCommand => {
                self.on = true;
            }
            Event::OffCommand => {
                self.on = false;
            }
            Event::Stop => {
                self.banger.reset();
                self.send_music();
            }
            Event::Play(p) => {
                if let Some(socket) = &mut self.socket {
                    socket.write_message(Message::Text(p.to_string()))
                        .expect(
                            "Socket connection cannot fail, or this entire program is doo doo garbage",
                        );
                }
            }
            Event::DrumCommand(d) => {
                if let Err(_) = self.banger.bang(&d) {
                    println!("Drum command failed {}", d);
                    return;
                }
                self.send_music();
            }
            _ => {}
        }
    }
}

struct TwitchSerializer {
    msg: Vec<String>,
}

impl BangersSerializer for TwitchSerializer {
    fn write(&mut self, node: WriteNode) {
        match node {
            WriteNode::Thing(drum, _, on) => {
                if on {
                    self.msg.push(format!("sample :{}", drum).to_string());
                }
            },
            WriteNode::ThingDone => {
                self.msg.push("sleep 0.25".to_string());
            }
        }
    }
}

impl TwitchSerializer {
    fn new() -> TwitchSerializer {
        return TwitchSerializer {
            msg: Vec::new(),
        }
    }

    fn to_string(&self) -> String {
        let mut msg = self.msg.clone();

        // TODO: Research VecDeque
        // What is it?
        msg.insert(0, "live_loop :bangers do".to_string());
        msg.insert(1, "    use_bpm 120".to_string());
        msg.push("end".to_string());

        return msg.join("\n");
    }
}

impl Client {
    fn send_music(&mut self) {
        if let Some(socket) = &mut self.socket {
            let mut serializer = TwitchSerializer::new();

            self.banger.serialize(Direction::Column, &mut serializer);

            let music = serializer.to_string();

            println!("music: {}", music);
            socket.write_message(Message::Text(music))
                .expect(
                    "Socket connection cannot fail, or this entire program is doo doo garbage",
                );
        }
    }
    pub fn new() -> Client {
        return Client {
            on: false,
            socket: None,
            banger: beatmedaddy::bangers::Bangers::new(),
        };
    }

    pub fn connect(&mut self, opts: Arc<PiOpts>) -> Result<(), Box<dyn std::error::Error>> {
        let (socket, _) =
            connect(Url::parse(format!("ws://{}:{}", opts.addr, opts.port).as_str()).unwrap())
                .expect("Can't connect");

        self.socket = Some(socket);

        return Ok(());
    }
}
