use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use std::fs;

use crate::Cursor;

#[derive(Debug)]
pub struct Buffer {
    file: String,
    file_content: Vec<String>,
}

impl Buffer {
    pub fn new(file: String) -> Self {
        let file_content = fs::read_to_string(&file)
            .expect("File {file} not found or could not be opened.")
            .split('\n')
            .map(|x| x.to_string())
            .collect();

        Self { file, file_content }
    }

    pub fn to_parragraph(&self, cursor: &mut Cursor) -> Paragraph {
        let file_len = self.file_content().len();
        let range = if cursor.y_copy() + crossterm::terminal::size().unwrap().1 as usize > file_len
        {
            cursor.y_copy()..file_len
        } else {
            cursor.y_copy()..cursor.y_copy() + crossterm::terminal::size().unwrap().1 as usize
        };

        let lines: Vec<Line> = self.file_content()[range]
            .iter()
            .enumerate()
            .map(|(y_index, line)| {
                if y_index == 0 {
                    let span: Vec<Span> = line
                        .chars()
                        .enumerate()
                        .map(|(x_index, cchar)| {
                            if x_index == *cursor.x_mut() {
                                return Span::from(cchar.to_string()).on_white().black();
                            }
                            Span::from(cchar.to_string())
                        })
                        .collect();
                    return Line::from(span);
                }
                Line::from(line.as_str())
            })
            .collect();

        Paragraph::new(lines)
    }

    pub fn file_content(&self) -> &[String] {
        &self.file_content
    }

    pub fn file_content_mut(&mut self) -> &mut Vec<String> {
        &mut self.file_content
    }

    pub fn file(&self) -> &str {
        &self.file
    }
}
