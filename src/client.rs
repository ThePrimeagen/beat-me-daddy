use crate::{
    event::Event,
    event_bus::Listener,
    opt::PiOpts, bangers::Bangers,
};
use std::{sync::Arc, net::ToSocketAddrs};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

pub struct Client {
    on: bool,
    socket: Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>>,
    banger: Bangers,
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
            Event::Play(p) => {
                if let Some(socket) = &mut self.socket {
                    socket.write_message(Message::Text(p.to_string()))
                        .expect(
                            "Socket connection cannot fail, or this entire program is doo doo garbage",
                        );
                }
            }
            Event::DrumCommand(d) => {
                if !self.on {
                    return;
                }

                self.banger.bang(&d).expect("This not to fail, don't do it chat or your ass is banned");

                if let Some(socket) = &mut self.socket {
                    let music = self.banger.serialize();
                    socket.write_message(Message::Text(music))
                        .expect(
                            "Socket connection cannot fail, or this entire program is doo doo garbage",
                        );
                }
            }
            _ => {}
        }
    }
}

impl Client {
    pub fn new() -> Client {
        return Client {
            on: false,
            socket: None,
            banger: Bangers::new(),
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
