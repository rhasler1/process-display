use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use super::{EventState, StatefulDrawableComponent, Component};

// FilterComponent stores the characters entered by the user in the filter bar.
pub struct FilterComponent {
    filter: String,
}

impl FilterComponent {
    pub fn new() -> Self {
        Self {
            filter: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.filter.clear();
    }

    // pub method to get the current filter
    // The method is used in the impl of App to communicate the filter with the CPUComponent.
    //
    pub fn get_filter(&mut self) -> String {
        return self.filter.clone();
    }

    pub fn is_filter_empty(&mut self) -> bool {
        return self.filter.is_empty();
    }
}

impl Component for FilterComponent {
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        match key.code {
            KeyCode::Char(c) => {
                self.filter.push(c);
                Ok(EventState::Consumed)
            }
            KeyCode::Backspace => {
                self.filter.pop();
                Ok(EventState::Consumed)
            }
            _ => Ok(EventState::NotConsumed)
        }
    }
}

impl StatefulDrawableComponent for FilterComponent {
    fn draw(&mut self, f: &mut Frame, area: ratatui::prelude::Rect) -> io::Result<()> {
        let widget: Paragraph = Paragraph::new(self.filter.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Filter by process name"));
        f.render_widget(widget, area);
        return Ok(())
    }
}