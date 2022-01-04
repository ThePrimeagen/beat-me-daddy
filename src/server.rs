use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use std::process::Command;
use std::sync::Arc;

use crate::opt::PiOpts;

pub fn server(opts: Arc<PiOpts>) -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind(format!("{}:{}", opts.addr, opts.port))?;
    if opts.debug {
        println!("Server listening on {}", opts.port);
    }

    for stream in server.incoming() {
        let opts = opts.clone();
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            println!("Connection accepted");

            while let Ok(msg) = websocket.read_message() {
                // We do not want to send back ping/pong messages.
                if msg.is_text() {
                    println!("music: {}", msg);

                    Command::new(&opts.command)
                        .arg(msg.to_text().unwrap())
                        .output()
                        .expect("failed to execute process");
                }
            }
        });
    }

    return Ok(());
}
