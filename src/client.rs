use tungstenite::{connect, Message};
use url::Url;
use crate::opt::PiOpts;
use std::sync::Arc;

pub fn client(opts: Arc<PiOpts>) -> Result<(), Box<dyn std::error::Error>> {
    let (mut socket, response) =
        connect(Url::parse(format!("ws://{}:{}", opts.addr, opts.port).as_str()).unwrap())
            .expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    loop {
        socket.write_message(Message::Text("Hello WebSocket".into()))?;
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }

    // socket.close(None);

}
