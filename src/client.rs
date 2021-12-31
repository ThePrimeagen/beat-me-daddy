use crate::{
    event::Event,
    event_bus::Listener,
    opt::PiOpts
};

use std::sync::Arc;
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

impl Client {
    fn send_music(&mut self) {
        if let Some(socket) = &mut self.socket {
            let music = self.banger.serialize();
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
