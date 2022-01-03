use beatmedaddy::{
    event::Event,
    event_bus::Listener,
};
use crate::opt::PiOpts;

use std::sync::Arc;
use beatmedaddy::bangers::bangers::{WriteNode, BangersSerializer, Direction};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

pub struct Client {
    on: bool,
    socket: Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>>,
    banger: beatmedaddy::bangers::bangers::Bangers,
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

struct SonicPiSerializer {
    msg: Vec<String>,
}

impl BangersSerializer for SonicPiSerializer {
    fn direction(&self) -> Direction {
        return Direction::Column;
    }

    fn write(&mut self, node: WriteNode) {
        match node {
            WriteNode::ThingFinished => {}

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

impl SonicPiSerializer {
    fn new() -> SonicPiSerializer {
        return SonicPiSerializer {
            msg: Vec::new(),
        }
    }
}

impl std::fmt::Display for SonicPiSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("live_loop :bangers do\n")?;
        f.write_str("    use_bpm 120\n")?;
        for line in &self.msg {
            f.write_str(line)?;
            f.write_str("\n")?;
        }

        f.write_str("end")?;

        Ok(())
    }
}

impl Client {
    fn send_music(&mut self) {
        if let Some(socket) = &mut self.socket {
            let mut serializer = SonicPiSerializer::new();

            self.banger.serialize(&mut serializer);

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
            banger: beatmedaddy::bangers::bangers::Bangers::new(),
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
