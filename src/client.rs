use crate::{
    event::Event,
    event_bus::Listener,
    opt::PiOpts,
};
use std::sync::Arc;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

pub struct Client {
    on: bool,
    socket: Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>>,
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
            Event::DrumCommand(d) => {
                if let Some(socket) = &mut self.socket {
                    socket.write_message(Message::Text(format!("sample :{}", d).into()))
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
