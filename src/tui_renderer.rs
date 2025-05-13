use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
};

use crate::{Buffer, Cursor};

#[derive(Debug)]
pub struct TuiRenderer {
    buffer: Buffer,
    cursor: Cursor,
}

impl TuiRenderer {
    pub fn new(buffer: Buffer) -> Self {
        let cursor = Cursor::new(0, 0);
        Self { buffer, cursor }
    }

    pub fn run(&mut self) {
        let mut terminal = ratatui::init();
        loop {
            self.draw(&mut terminal);
            let Ok(x) = event::read() else { continue };
            if let Event::Key(key) = x {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char('l') => *self.cursor.x_mut() += 1,
                    event::KeyCode::Char('k') => *self.cursor.y_mut() -= 1,
                    event::KeyCode::Char('j') => *self.cursor.y_mut() += 1,
                    event::KeyCode::Char('h') => *self.cursor.x_mut() -= 1,
                    _ => continue,
                }
            }
        }
        ratatui::restore();
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) {
        terminal
            .draw(|frame| {
                let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(2)]);
                let [main_area, status_area] = vertical.areas(frame.area());

                let main_text = self
                    .buffer
                    .to_parragraph(&mut self.cursor)
                    .bg(Color::Rgb(40, 44, 52));

                let status_text = Paragraph::new("Normal")
                    .block(
                        Block::bordered()
                            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                            .title("Status"),
                    )
                    .bg(Color::Rgb(40, 44, 52));

                frame.render_widget(main_text, main_area);
                frame.render_widget(status_text, status_area);
            })
            .expect("failed to draw frame");
    }
}
