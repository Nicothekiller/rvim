use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
};

use crate::Buffer;

#[derive(Debug)]
pub struct TuiRenderer {
    buffer: Buffer,
}

impl TuiRenderer {
    pub fn new(buffer: Buffer) -> Self {
        Self { buffer }
    }

    pub fn run(&self) {
        let mut terminal = ratatui::init();
        loop {
            self.draw(&mut terminal);
            if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
                break;
            }
        }
        ratatui::restore();
    }

    fn draw(&self, terminal: &mut DefaultTerminal) {
        terminal
            .draw(|frame| {
                let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(2)]);
                let [main_area, status_area] = vertical.areas(frame.area());

                let main_text = self.buffer.to_parragraph().bg(Color::Rgb(40, 44, 52));

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
