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
        let lines: Vec<Line> = self
            .file_content()
            .iter()
            .enumerate()
            .map(|(y_index, line)| {
                if y_index == *cursor.y_mut() {
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

        if cursor.y_copy() + crossterm::terminal::size().unwrap().1 as usize > lines.len() {
            let text: Vec<Line> = lines[cursor.y_copy()..lines.len()].to_vec();
            Paragraph::new(text)
        } else {
            let text: Vec<Line> = lines[cursor.y_copy()
                ..cursor.y_copy() + crossterm::terminal::size().unwrap().1 as usize]
                .to_vec();
            Paragraph::new(text)
        }
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
