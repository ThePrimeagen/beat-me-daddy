use beatmedaddy::bangers::Bangers;
use std::io::{self, Stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Text, Spans};
use tui::widgets::{Widget, Block, Borders, BorderType, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction, Alignment, Rect};
use crate::util::event::{Events, Event};

const UNSELECTED: &str = "░";
const SELECTED: &str = "█";

pub struct UI {
    bangers: Bangers,
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
}

impl UI {
    pub fn new() -> Result<UI, Box<dyn std::error::Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        return Ok(UI {
            bangers: Bangers::new(),
            terminal ,
        });
    }

    pub fn key(&mut self, key: Key) -> Result<(), Box<dyn std::error::Error>> {
        return Ok(());
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        return self.render();
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            let size = f.size();
            let text = vec![
                Spans::from("This is a line "),
                Spans::from(Span::styled("This is a line   ", Style::default().fg(Color::Red))),
                Spans::from(Span::styled("This is a line", Style::default().bg(Color::Blue))),
                Spans::from(Span::styled(
                        "This is a longer line\n",
                        Style::default().add_modifier(Modifier::CROSSED_OUT),
                )),
                Spans::from(Span::styled(
                        "This is a line\n",
                        Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC),
                )),
            ];

            let paragraph = Paragraph::new(text.clone())
                .block(Block::default().title("Left Block"))
                .alignment(Alignment::Left).wrap(Wrap { trim: true });
            f.render_widget(paragraph, size);
        })?;

        return Ok(());
    }
}
