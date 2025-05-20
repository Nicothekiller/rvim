use std::{fs, io::Write};

use crossterm::event;
use crossterm::event::Event;
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
};

use crate::{Cursor, FileBuffer};

#[derive(Debug, PartialEq)]
pub enum EditingMode {
    Normal,
    Insert,
}

/// Struct for handling and rendering the tui. Should be used by creating a new [`TuiRenderer`] and using [`TuiRenderer::run`].
#[derive(Debug)]
pub struct TuiRenderer {
    buffer: FileBuffer,
    cursor: Cursor,
    mode: EditingMode,
    should_quit: bool,
}

impl TuiRenderer {
    /// Creates a new [`TuiRenderer`].
    pub fn new(buffer: FileBuffer) -> Self {
        let cursor = Cursor::new(0, 0);
        let mode = EditingMode::Normal;
        Self {
            buffer,
            cursor,
            mode,
            should_quit: false,
        }
    }

    /// Starts the tui and handles the input.
    pub fn run(&mut self) {
        let mut terminal = ratatui::init();
        while !self.should_quit {
            self.draw(&mut terminal);
            self.handle_input();
        }
        ratatui::restore();
    }

    /// Handles the input from the terminal and translates it to the appropriate actions in the tui.
    /// Each editing mode has a function for handling it.
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

    /// Handles the input for insert mode.
    fn insert_ih(&mut self, key: event::KeyEvent) {
        match key.code {
            event::KeyCode::Esc => self.mode = EditingMode::Normal,
            event::KeyCode::Char(x) => {
                self.buffer.file_content_mut()[self.cursor.y_copy()]
                    .insert(self.cursor.x_copy(), x);
                *self.cursor.x_mut() += 1;
            }
            event::KeyCode::Backspace => {
                self.buffer.file_content_mut()[self.cursor.y_copy()]
                    .remove(self.cursor.x_copy() - 1);
                *self.cursor.x_mut() -= 1;
            }
            _ => {}
        }
    }

    ///Handles the input for normal mode.
    fn normal_ih(&mut self, key: event::KeyEvent) {
        match key.code {
            event::KeyCode::Char('q') => {
                self.should_quit = true;
                let file_content = self.buffer.file_content().join("\n");
                fs::write(self.buffer.file(), file_content).expect("Unable to write file.");
            }
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

    /// Responsible for drawing the tui.
    ///
    /// # Panics
    ///
    /// Panics if [ratatui::terminal::Terminal::draw] function fails. This should not be an issue.
    fn draw(&mut self, terminal: &mut DefaultTerminal) {
        terminal
            .draw(|frame| {
                let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(2)]);
                let [main_area, status_area] = vertical.areas(frame.area());

                let main_text = self
                    .buffer
                    .to_paragraph(&mut self.cursor)
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
