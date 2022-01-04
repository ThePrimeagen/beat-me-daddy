use std::sync::{Arc, Mutex};
use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle};

use crate::event::Event;

pub trait Listener {
    fn notify(&mut self, event: &Event);
}

pub trait Dispatchable {
    fn register_listener(&mut self, listener: Arc<Mutex<dyn Listener + Send>>);
}

#[derive(Default)]
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

pub fn run_dispatcher(mut rx: UnboundedReceiver<crate::event::Event>, mut dispatcher: Dispatcher) -> JoinHandle<()> {
    let output_handle = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            for l in dispatcher.listeners.iter_mut() {
                l.lock().expect("Dumb ass kid back home").notify(&message.clone());
            }
        }

        return;
    });

    return output_handle;
}
