use futures_channel::mpsc::UnboundedSender;
use super::runners::{Runner, Debug, PlayTheThing, TurnMeDaddy};

use crate::event_bus::Listener;
use crate::event::Event;

pub struct PrimeListener {
    runners: Vec<Box<dyn Runner + Send>>,
}

impl Listener for PrimeListener {
    fn notify(&mut self, event: &Event) {
        for runner in &mut self.runners {
            if runner.matches(event) {
                break;
            }
        }
    }
}

impl PrimeListener {
    pub fn new(tx: UnboundedSender<Event>) -> PrimeListener {
        let runners: Vec<Box<dyn Runner + Send>> = vec![
            Box::new(PlayTheThing { tx: tx.clone() }),
            Box::new(TurnMeDaddy { tx: tx.clone() }),
            Box::new(Debug {}),
        ];

        return PrimeListener {
            runners
        };
    }
}
