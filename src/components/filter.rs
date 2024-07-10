use std::io;

use crossterm::event::{KeyEvent, KeyCode};

use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use super::{EventState, StatefulDrawableComponent, Component};

pub struct FilterComponent {
    input_str: String,
}

impl FilterComponent {
    pub fn new() -> Self {
        Self {
            input_str: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.input_str.clear();
    }

    pub fn input_str(&mut self) -> String {
        return self.input_str.clone();
    }

    pub fn is_filter_empty(&mut self) -> bool {
        return self.input_str.is_empty();
    }
}

impl Component for FilterComponent {
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        match key.code {
            KeyCode::Char(c) => {
                self.input_str.push(c);
                Ok(EventState::Consumed)
            }
            KeyCode::Backspace => {
                self.input_str.pop();
                Ok(EventState::Consumed)
            }
            _ => Ok(EventState::NotConsumed)
        }
    }
}

impl StatefulDrawableComponent for FilterComponent {
    fn draw(&mut self, f: &mut Frame, area: ratatui::prelude::Rect, focused: bool) -> io::Result<()> {
        let title: &str = "Filter";

        let style: Style =
        if focused {
            Style::default().fg(Color::White)
        }
        else {
            Style::default().fg(Color::DarkGray)
        };

        let filter_text: &str = self.input_str.as_str();

        let widget: Paragraph =
            Paragraph::new(filter_text)
            .style(style)
            .block(Block::default().borders(Borders::ALL)
            .title(title));

        f.render_widget(widget, area);

        return Ok(())
    }
}