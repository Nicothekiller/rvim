use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
};

use crate::{Buffer, Cursor};

#[derive(Debug, PartialEq)]
pub enum EditingMode {
    Normal,
    Insert,
}

#[derive(Debug)]
pub struct TuiRenderer {
    buffer: Buffer,
    cursor: Cursor,
    mode: EditingMode,
    should_quit: bool,
}

impl TuiRenderer {
    pub fn new(buffer: Buffer) -> Self {
        let cursor = Cursor::new(0, 0);
        let mode = EditingMode::Normal;
        Self {
            buffer,
            cursor,
            mode,
            should_quit: false,
        }
    }

    pub fn run(&mut self) {
        let mut terminal = ratatui::init();
        while !self.should_quit {
            self.draw(&mut terminal);
            self.handle_input();
        }
        ratatui::restore();
    }

    fn handle_input(&mut self) {
        let Ok(x) = event::read() else { return };

        if let Event::Key(key) = x {
            match self.mode {
                EditingMode::Normal => {
                    self.normal_ih(key);
                }
                EditingMode::Insert => {
                    self.insert_ih(key);
                }
            }
        }
    }

    fn insert_ih(&mut self, key: event::KeyEvent) {
        if key.code == event::KeyCode::Esc {
            self.mode = EditingMode::Normal
        }
    }

    fn normal_ih(&mut self, key: event::KeyEvent) {
        match key.code {
            event::KeyCode::Char('q') => self.should_quit = true,
            event::KeyCode::Char('l') => *self.cursor.x_mut() += 1,
            event::KeyCode::Char('k') => {
                if *self.cursor.y_mut() != 0 {
                    *self.cursor.y_mut() -= 1
                }
            }
            event::KeyCode::Char('j') => *self.cursor.y_mut() += 1,
            event::KeyCode::Char('h') => {
                if *self.cursor.x_mut() != 0 {
                    *self.cursor.x_mut() -= 1
                }
            }
            event::KeyCode::Char('i') => self.mode = EditingMode::Insert,
            _ => {}
        }
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

                let status_text_mode = match self.mode {
                    EditingMode::Normal => "NORMAL",
                    EditingMode::Insert => "INSERT",
                };

                let status_text = Paragraph::new(status_text_mode)
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
