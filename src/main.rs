mod mailbox;

use crate::mailbox::constants::E8;
use crate::mailbox::pieces::Square;
use crate::mailbox::pieces::{Piece, PieceType};
use crate::mailbox::read_positions;
use crate::mailbox::DisplayBoard;
use crate::mailbox::MailboxBoard;
use crate::mailbox::KNIGHTS_TOUR;
use core::fmt::Write;
use core::time::Duration;
use crossterm::terminal::EnterAlternateScreen;
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::Margin;
use tui::widgets::Widget;
use tui::Terminal;

use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
};

use color_eyre::Result;

pub trait Board {
    fn display(&self) -> DisplayBoard;
}

impl Board for MailboxBoard {
    fn display(&self) -> DisplayBoard {
        self.display()
    }
}

impl Widget for DisplayBoard {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let mut s = String::new();
        write!(s, "{}", self);
        for (i, line) in s.lines().enumerate() {
            buf.set_string(area.left(), area.top() + i as u16, line, Default::default())
        }
    }
}

fn render<B: Board>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    board: &B,
) -> Result<()> {
    terminal.draw(|f| {
        // Wrapping block for a group
        // Just draw the block and the group on the same area and build the group
        // with at least a margin of 1
        let size = f.size();

        // Surrounding block
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Main block with round corners")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, size);
        f.render_widget(
            board.display(),
            size.inner(&Margin {
                vertical: 2,
                horizontal: 2,
            }),
        );
    })?;
    Ok(())
}

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::enable_raw_mode;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut mailbox = MailboxBoard::default();
    mailbox.set(Square::Occupied(Piece::White(PieceType::Knight)), E8);
    let tour = read_positions(KNIGHTS_TOUR);
    let mut tour = tour.iter();

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;

    let tick_rate = Duration::from_millis(250);

    let mut prev_pos = E8;
    loop {
        let next_pos = tour.next().unwrap();
        mailbox.set(Square::Empty, prev_pos);
        mailbox.set(Square::Occupied(Piece::White(PieceType::Knight)), *next_pos);
        prev_pos = *next_pos;
        render(&mut terminal, &mailbox)?;
        if let Ok(true) = event::poll(tick_rate) {
            let event = event::read().unwrap();
            match event {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
    }
}
