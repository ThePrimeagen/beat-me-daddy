use std::sync::{Arc, Mutex};
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::StreamExt;
use tokio::{task::JoinHandle};

use crate::event::Event;

pub trait Listener {
    fn notify(&mut self, event: &Event);
}

pub trait Dispatchable {
    fn register_listener(&mut self, listener: Arc<Mutex<dyn Listener + Send>>);
}

pub struct Dispatcher {
    /// A list of synchronous weak refs to listeners
    listeners: Vec<Arc<Mutex<dyn Listener + Send>>>,
}

impl Dispatchable for Dispatcher {
    /// Registers a new listener
    fn register_listener(&mut self, listener: Arc<Mutex<dyn Listener + Send>>) {
        self.listeners.push(listener);
    }
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        return Dispatcher {
            listeners: Vec::new(),
        };
    }
}

pub fn run_dispatcher(mut rx: UnboundedReceiver<crate::event::Event>, mut dispatcher: Dispatcher) -> JoinHandle<()> {
    let output_handle = tokio::spawn(async move {
        loop {
            println!("wating on run_dispatech");
            if let Some(message) = rx.next().await {
                println!("Received message! {:?}", message);
                for l in dispatcher.listeners.iter_mut() {
                    l.lock().expect("listener lock shouldn't ever fail").notify(&message.clone());
                }
            }
        }
    });

    return output_handle;
}
