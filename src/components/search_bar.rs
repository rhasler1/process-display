use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use super::StatefulDrawableComponent;
use super::Component;

// Currently, the program can only search for a process when provided a process name.
pub struct SearchBar {
    process_name: String,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            process_name: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.process_name.clear();
    }

    pub fn get_process_name(&mut self) -> String {
        return self.process_name.clone();
    }

    pub fn is_empty(&mut self) -> bool {
        return self.process_name.is_empty();
    }
}

impl Component for SearchBar {
    fn event(&mut self, key: KeyEvent, _filter: Option<String>) -> io::Result<bool> {
        match key.code {
            KeyCode::Char(c) => {
                self.process_name.push(c);
                Ok(true) // key event consumed
            }
            KeyCode::Backspace => {
                self.process_name.pop();
                Ok(true) // key event consumed
            }
            _ => Ok(false) // key event not consumed
        }
    }
}

impl StatefulDrawableComponent for SearchBar {
    fn draw(&mut self, f: &mut Frame, area: ratatui::prelude::Rect) -> io::Result<bool> {
        let widget: Paragraph = Paragraph::new(self.process_name.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Search by name"));
        f.render_widget(widget, area);
        return Ok(true)
    }
}