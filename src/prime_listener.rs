use crate::{event::Event, event_bus::Listener};

pub struct PrimeListener { }

impl Listener for PrimeListener {
    fn notify(&mut self, event: &Event) {
        println!("prime listener event {:?}", event);
    }
}

impl PrimeListener {
    pub fn new() -> PrimeListener {
        return PrimeListener {};
    }
}
