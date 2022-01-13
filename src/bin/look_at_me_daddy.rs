mod util;
mod ui;

use termion::event::Key;
use ui::opts::UiConfig;
use util::event::{Events, Event};
use beatmedaddy::twitch::twitch_client::Twitch;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    let opts = UiConfig::from_args();

    let events = Events::new();
    let twitch = Twitch::new();
    let mut ui = ui::ui::UI::new(Some(twitch), opts)?;

    loop {
        match events.next() {
            Ok(Event::Input(Key::Ctrl('c'))) => break,
            Ok(Event::Input(c)) => ui.key(c).await,
            Ok(Event::Tick) => ui.tick(),
            _ => continue
        }?;
    }

    return Ok(());
}

