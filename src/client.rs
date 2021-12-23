use tungstenite::{connect, Message};
use url::Url;
use crate::opt::PiOpts;
use std::sync::Arc;

pub fn client(opts: Arc<PiOpts>) -> Result<(), Box<dyn std::error::Error>> {
    let (mut socket, response) =
        connect(Url::parse(format!("ws://{}:{}", opts.addr, opts.port).as_str()).unwrap())
            .expect("Can't connect");

    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    // loop {
        socket.write_message(Message::Text("sample :bass_dnb_f".into()))?;
    // }

    // socket.close(None);
    //
    return Ok(());
}
