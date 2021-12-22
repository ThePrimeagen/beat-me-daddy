use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

use crate::opt::PiOpts;

pub fn server(opts: PiOpts) {
    let server = TcpListener::bind(format!("{}:{}", opts.addr, opts.port)).unwrap();
    if opts.debug {
        println!("Server listening on {}", opts.port);
    }

    for stream in server.incoming() {
        println!("New connection");
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            println!("Connection accepted");
            loop {
                let msg = match websocket.read_message() {
                    Ok(m) => m,
                    Err(_) => {
                        break;
                    },
                };

                println!("Message from socket: {:?}", msg);

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
