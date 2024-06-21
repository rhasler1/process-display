use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use super::{EventState, StatefulDrawableComponent};
use super::Component;

// Currently, the program can only search for a process when provided a process name.
pub struct ProcessFilter {
    filter_name: String,
}

impl ProcessFilter {
    pub fn new() -> Self {
        Self {
            filter_name: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.filter_name.clear();
    }

    pub fn get_filter_name(&mut self) -> String {
        return self.filter_name.clone();
    }

    pub fn is_empty(&mut self) -> bool{
        return self.filter_name.is_empty();
    }
}

impl Component for ProcessFilter {
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        match key.code {
            KeyCode::Char(c) => {
                self.filter_name.push(c);
                Ok(EventState::Consumed) // key event consumed
            }
            KeyCode::Backspace => {
                self.filter_name.pop();
                Ok(EventState::Consumed) // key event consumed
            }
            _ => Ok(EventState::NotConsumed) // key event not consumed
        }
    }
}

impl StatefulDrawableComponent for ProcessFilter {
    fn draw(&mut self, f: &mut Frame, area: ratatui::prelude::Rect) -> io::Result<()> {
        let widget: Paragraph = Paragraph::new(self.filter_name.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Search by name"));
        f.render_widget(widget, area);
        return Ok(())
    }
}