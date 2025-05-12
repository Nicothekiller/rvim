use crossterm::event::{self, Event};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(2)]);
    let [main_area, status_area] = vertical.areas(frame.area());

    let text = Text::raw("Hello World!");
    let status_text = Paragraph::new("Normal").block(
        Block::bordered()
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
            .title("Status"),
    );
    frame.render_widget(text, main_area);
    frame.render_widget(status_text, status_area);
}
