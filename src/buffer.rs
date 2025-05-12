use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use std::fs;

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

    pub fn to_parragraph(&self) -> Paragraph {
        let lines: Vec<Line> = self
            .file_content()
            .iter()
            .map(|x| Line::from(x.as_str()))
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

