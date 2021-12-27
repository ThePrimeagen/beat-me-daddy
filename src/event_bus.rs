use std::sync::{Arc, Mutex};
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
        Dispatcher { listeners: Vec::new() }
    }

    pub fn dispatch(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        for l in self.listeners.iter() {
            // TODO: I don't even get this
            // TODO AT SOME POINT: Understand this
            let mut listener = l.lock().expect("listener lock should never fail");
            listener.notify(&event);
        }

        return Ok(());
    }
}
