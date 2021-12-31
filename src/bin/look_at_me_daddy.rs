mod util;
mod ui;

use termion::event::Key;
use util::event::{Events, Event};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let events = Events::new();
    let mut ui = ui::ui::UI::new()?;

    loop {
        match events.next() {
            Ok(Event::Input(Key::Ctrl('c'))) => break,
            Ok(Event::Input(c)) => ui.key(c),
            Ok(Event::Tick) => ui.tick(),
            _ => continue
        }
    }

    return Ok(());
}

