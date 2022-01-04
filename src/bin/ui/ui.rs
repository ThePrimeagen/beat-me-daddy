use beatmedaddy::bangers::bangers::{self, Bangers, BangersSerializer, WriteNode};
use beatmedaddy::bangers::boolizer::Boolizer;
use beatmedaddy::bangers::consts::{BEAT_COUNT, STARTING_UTF};
use beatmedaddy::twitch::twitch_client::Twitch;
use std::collections::HashMap;
use std::io::{self, Stdout};
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Paragraph, Wrap};
use tui::Terminal;

const SEPARATOR: &str = "░";
const UNSELECTED: &str = "▒";
const SELECTED: &str = "█";

pub type PrimeResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct UI {
    twitch: Option<Twitch>,
    bangers: Bangers,
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    cursor: Cursor,
    title: String,
}

struct UIBangerSerializer {
    drums: HashMap<String, [bool; BEAT_COUNT]>,
}

// TODO: Do I like the cursor moving itself?  Or just have it dumb?
struct Cursor {
    drum_idx: usize,
    column: usize,
}

impl Cursor {
    fn new() -> Cursor {
        return Cursor {
            drum_idx: 0,
            column: 0,
        };
    }

    fn j(&mut self) {
        self.drum_idx = (self.drum_idx + 1) % Bangers::get_keys().len();
    }

    fn k(&mut self) {
        if self.drum_idx == 0 {
            self.drum_idx = Bangers::get_keys().len() - 1;
        } else {
            self.drum_idx = self.drum_idx - 1;
        }
    }

    #[allow(non_snake_case)]
    fn A(&mut self) {
        self.column = BEAT_COUNT - 1;
    }

    #[allow(non_snake_case)]
    fn I(&mut self) {
        self.column = 0;
    }

    fn l(&mut self) {
        self.column = (self.column + 1).min(BEAT_COUNT - 1);
    }

    fn h(&mut self) {
        self.column = self.column.saturating_sub(1);
    }

    fn w(&mut self) {
        self.column = ((self.column + 4) & (!3)).min(BEAT_COUNT - 1);
    }

    #[allow(non_snake_case)]
    fn W(&mut self) {
        self.column = ((self.column + 16) & (!15)).min(BEAT_COUNT - 1);
    }

    fn b(&mut self) {
        self.column = self.column.saturating_sub(1) & (!3);
    }

    #[allow(non_snake_case)]
    fn B(&mut self) {
        self.column = self.column.saturating_sub(1) & (!15);
    }

    fn at_drum(&self, drum_idx: usize) -> bool {
        return self.drum_idx == drum_idx;
    }

    fn at(&self, drum_idx: usize, column: usize) -> bool {
        return self.drum_idx == drum_idx && self.column == column;
    }
}

impl UIBangerSerializer {
    fn new() -> UIBangerSerializer {
        return UIBangerSerializer {
            drums: HashMap::new(),
        };
    }

    fn drums_to_spans<'a>(&mut self, order: &'static [&'static str], cursor: &Cursor) -> Vec<Spans<'a>> {
        let mut out: Vec<Spans<'a>> = Vec::new();

        for (idx, drum) in order.iter().enumerate() {
            if cursor.at_drum(idx) {
                // TODO: Fix this and make it pretty
                out.push(Spans::from(drum.to_string()));
            } else {
                out.push(Spans::from(drum.to_string()));
            }
        }

        return out;
    }

    // TODO: the bounds of the ui
    // TODO: ^
    fn to_spans<'a>(&mut self, order: &'static [&'static str], cursor: &Cursor) -> Vec<Spans<'a>> {
        let mut out: Vec<Spans<'a>> = Vec::new();

        // How do i place cursor?
        for (idx, drum) in order.iter().enumerate() {
            let span_list = self.drums
                .entry(drum.to_string())
                .or_insert([false; BEAT_COUNT])
                .iter()
                .enumerate()
                .map(|(col, x)| {
                    let note = if *x { SELECTED } else { UNSELECTED };
                    if cursor.at(idx, col) {
                        return Span::styled(note, Style::default().fg(Color::Red))
                    } else if col % 4 == 0 {
                        return Span::styled(note, Style::default().fg(Color::DarkGray))
                    }
                    return Span::from(note);
                })
                .collect::<Vec<Span>>();

                out.push(Spans::from(span_list));
        }

        return out;
    }
}

struct TwitchSerializer {
    data: Boolizer,
}

impl TwitchSerializer {
    fn new() -> TwitchSerializer {
        return TwitchSerializer {
            data: Boolizer::new()
        }
    }

    fn to_twitch_string(&self) -> PrimeResult<String> {
        return Ok(STARTING_UTF.to_string() + &self.data.encode()?);
    }
}

impl BangersSerializer for TwitchSerializer {
    fn direction(&self) -> bangers::Direction {
        return bangers::Direction::Row;
    }

    fn write(&mut self, node: WriteNode) {
        match node {
            WriteNode::Thing(.., on) => {
                self.data.push(on).expect("This should never fail, said me once before.");
            }

            WriteNode::ThingDone => {}
            WriteNode::ThingFinished => { }
        }
    }
}

impl BangersSerializer for UIBangerSerializer {

    fn direction(&self) -> bangers::Direction {
        return bangers::Direction::Row;
    }

    fn write(&mut self, node: WriteNode) {
        match node {
            WriteNode::Thing(drum, pos, on) => {
                self.drums.entry(drum).or_insert([false; BEAT_COUNT])[pos] = on;
            }
            WriteNode::ThingDone => {}
            WriteNode::ThingFinished => {}
        }
    }
}

macro_rules! call_cursor {
    ($self:expr, $x:ident) => {
        {
            $self.cursor.$x();
            $self.render()?;
        }
    };
}

impl UI {
    pub fn new(twitch: Option<Twitch>) -> Result<UI, Box<dyn std::error::Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        return Ok(UI {
            bangers: Bangers::new(),
            terminal,
            twitch,
            cursor: Cursor::new(),
            title: "Sugma".to_string(),
        });
    }

    // TODO: Look at the anyhow crate
    pub async fn key(&mut self, key: Key) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            Key::Char('_') => call_cursor!(self, A),
            Key::Char('$') => call_cursor!(self, I),
            Key::Char('I') => call_cursor!(self, I),
            Key::Char('l') => call_cursor!(self, l),
            Key::Char('h') => call_cursor!(self, h),
            Key::Char('B') => call_cursor!(self, B),
            Key::Char('b') => call_cursor!(self, b),
            Key::Char('W') => call_cursor!(self, W),
            Key::Char('w') => call_cursor!(self, w),
            Key::Char('j') => call_cursor!(self, j),
            Key::Char('k') => call_cursor!(self, k),
            Key::Char(' ') => {
                self.bangers.toggle(self.cursor.drum_idx, self.cursor.column);
                self.render()?;
            }
            Key::Char('\n') => {
                let mut serializer = TwitchSerializer::new();
                self.bangers.serialize(&mut serializer);
                self.title = serializer.to_twitch_string()?;
                if let Some(twitch) = &mut self.twitch {
                    twitch.send_message(self.title.clone()).await;
                }
            }
            _ => {}
        }
        return Ok(());
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        return self.render();
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            let size = f.size();

            let top = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(2), Constraint::Min(12)].as_ref())
                .split(size);

            let title = Paragraph::new(
                Span::styled(self.title.clone(), Style::default().fg(Color::Red))
            )
                .block(Block::default())
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            f.render_widget(title, top[0]);

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(18), Constraint::Min(12)].as_ref())
                .split(top[1]);

            let mut serializer = UIBangerSerializer::new();
            self.bangers
                .serialize(&mut serializer);
            let drum_lines = serializer.to_spans(Bangers::get_keys(), &self.cursor);
            let drums = serializer.drums_to_spans(Bangers::get_keys(), &self.cursor);

            let drum_names = Paragraph::new(drums)
                .block(Block::default().title("Drums"))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(drum_names, chunks[0]);

            let paragraph = Paragraph::new(drum_lines)
                .block(Block::default().title("Tracks"))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunks[1]);
        })?;

        return Ok(());
    }
}
