use std::fs;

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
            event::KeyCode::Esc => {
                if self.cursor.x_copy() != 0 {
                    *self.cursor.x_mut() -= 1;
                }
                self.mode = EditingMode::Normal
            }
            event::KeyCode::Char(x) => {
                self.buffer.file_content_mut()[self.cursor.y_copy()]
                    .insert(self.cursor.x_copy(), x);
                *self.cursor.x_mut() += 1;
            }
            //for removing characters
            event::KeyCode::Backspace => {
                let y_copy = self.cursor.y_copy();
                let x_copy = self.cursor.x_copy();
                if x_copy != 0 {
                    self.buffer.file_content_mut()[y_copy].remove(x_copy - 1);
                    *self.cursor.x_mut() -= 1;
                } else {
                    // If cursor is at the start of the line, for removing the line and moving
                    // content upwards. Uses unsafe because program is single threaded and the borrow
                    // checker was being annoying. It should only be dangerous if mutli-threading is
                    // added to file editing.
                    let line_size = self.buffer.file_content()[y_copy - 1].len();
                    unsafe {
                        if self.cursor.y_copy() != 0 {
                            let file_content_mut: *mut Vec<String> = self.buffer.file_content_mut();
                            (*file_content_mut)[y_copy - 1].push_str(&(*file_content_mut)[y_copy]);
                            (*file_content_mut).remove(y_copy);
                        }
                    }
                    *self.cursor.y_mut() -= 1;
                    *self.cursor.x_mut() = line_size;
                }
            }
            event::KeyCode::Enter => {
                let y_copy = self.cursor.y_copy();
                let x_copy = self.cursor.x_copy();
                let line_size = self.buffer.file_content()[y_copy].len();

                // Similar case to the Backspace unsafe handler. For entering new lines. Uses unsafe
                // because the borrow checker was being annoying. Only dangerous if mutli-threading
                // is added to file editing.
                unsafe {
                    let file_content_mut: *mut Vec<String> = self.buffer.file_content_mut();
                    (*file_content_mut).insert(
                        y_copy + 1,
                        (*file_content_mut)[y_copy][x_copy..line_size].to_string(),
                    );
                    (*file_content_mut)[y_copy].truncate(x_copy);
                }
                *self.cursor.y_mut() += 1;
                *self.cursor.x_mut() = 0;
            }
            event::KeyCode::Down => {
                if self.cursor.y_copy() + 1 != self.buffer.file_content().len() - 1 {
                    *self.cursor.y_mut() += 1;
                }
            }
            event::KeyCode::Up => {
                if self.cursor.y_copy() != 0 {
                    *self.cursor.y_mut() -= 1;
                }
            }
            event::KeyCode::Right => {
                if self.cursor.x_copy() != self.buffer.file_content()[self.cursor.y_copy()].len() {
                    *self.cursor.x_mut() += 1;
                }
            }
            event::KeyCode::Left => {
                if self.cursor.x_copy() != 0 {
                    *self.cursor.x_mut() -= 1;
                }
            }
            _ => {}
        }
    }

    ///Handles the input for normal mode.
    fn normal_ih(&mut self, key: event::KeyEvent) {
        match key.code {
            // TODO: Add proper command mode with wq command and delete q keybind in normal mode.
            event::KeyCode::Char('q') => {
                self.should_quit = true;
                let file_content = self.buffer.file_content().join("\n");
                fs::write(self.buffer.file(), file_content).expect("Unable to write file.");
            }
            event::KeyCode::Char('i') => {
                let x_copy = self.cursor.x_copy();
                let line_len = self.buffer.file_content()[self.cursor.y_copy()].len();

                if x_copy > line_len {
                    *self.cursor.x_mut() = line_len;
                }
                self.mode = EditingMode::Insert
            }
            event::KeyCode::Char('j') | event::KeyCode::Down => {
                if self.cursor.y_copy() + 1 != self.buffer.file_content().len() - 1 {
                    *self.cursor.y_mut() += 1;
                }
            }
            event::KeyCode::Char('k') | event::KeyCode::Up => {
                if self.cursor.y_copy() != 0 {
                    *self.cursor.y_mut() -= 1;
                }
            }
            event::KeyCode::Char('l') | event::KeyCode::Right => {
                if self.cursor.x_copy() != self.buffer.file_content()[self.cursor.y_copy()].len() {
                    *self.cursor.x_mut() += 1;
                }
            }
            event::KeyCode::Char('h') | event::KeyCode::Left => {
                let x_copy = self.cursor.x_copy();

                if x_copy != 0 {
                    let line_len = self.buffer.file_content()[self.cursor.y_copy()].len();

                    if x_copy > line_len {
                        *self.cursor.x_mut() = line_len - 1;
                    } else {
                        *self.cursor.x_mut() -= 1;
                    }
                }
            }
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
