use anyhow::{Ok, Result};
use crate::input::*;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};
use crate::config::Config;
use super::{
    EventState,
    DrawableComponent,
    Component
};

#[derive(Default)]
pub struct FilterComponent {
    input_str: String,
    cursor_position: usize,
    pub config: Config,
}

const CURSOR_STEP: usize = 1;

impl FilterComponent {
    pub fn new(config: Config) -> Self {
        Self {
            input_str: String::new(),
            cursor_position: 0,
            config,
        }
    }

    pub fn reset(&mut self) {
        self.input_str.clear();
    }

    pub fn input_str(&self) -> &str {
        &self.input_str
    }

    pub fn is_filter_empty(&self) -> bool {
        self.input_str.is_empty()
    }

    pub fn filter_contents(&self) -> Option<&str> {
        if self.input_str.is_empty() { return None }
        else { return Some(&self.input_str) }
    }

    fn increment_cursor_position(&mut self) {
        let input_length = self.input_str.len();
        let cursor_position = self.cursor_position;

        self.cursor_position = cursor_position
            .saturating_add(CURSOR_STEP)
            .min(input_length)
    }

    fn decrement_cursor_position(&mut self) {
        self.cursor_position = self.cursor_position.saturating_sub(CURSOR_STEP);
    }
}

impl Component for FilterComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState> {
        match key {
            Key::Char(c) => {
                self.input_str.insert(self.cursor_position, c);
                self.increment_cursor_position();
                Ok(EventState::Consumed)
            }
            Key::Backspace => {
                if !self.input_str.is_empty() && self.cursor_position != 0 {
                    self.input_str.remove(self.cursor_position.saturating_sub(1));
                    self.decrement_cursor_position();
                }
                Ok(EventState::Consumed)
            }
            Key::Left => {
                self.decrement_cursor_position();
                Ok(EventState::Consumed)
            }
            Key::Right => {
                self.increment_cursor_position();
                Ok(EventState::Consumed)
            }
            _ => Ok(EventState::NotConsumed)
        }
    }

    fn mouse_event(&mut self, _mouse: Mouse) -> Result<EventState> {
        Ok(EventState::NotConsumed)
    }
}

const TITLE: &str = " Filter ";

impl DrawableComponent for FilterComponent {
    fn draw(&mut self, f: &mut Frame, area: ratatui::prelude::Rect, focused: bool) -> Result<()> {
        let filter_input = self.input_str.clone();
        let cursor_position = self.cursor_position;
        let mut spans = Vec::new();
        
        // styling filter input w/ cursor position
        // only style w/ cursor if in filter is in focus
        for (i, ch) in filter_input.chars().enumerate() {
            let span = if i == cursor_position && focused {
                Span::styled(
                    ch.to_string(),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                )
            }
            else {
                Span::styled(
                    ch.to_string(),
                    Style::default()
                        .fg(Color::White)
                )
            };
            spans.push(span);
        }

        if cursor_position == filter_input.len() && focused {
            spans.push(
                Span::styled(" ", Style::default().bg(Color::White))
            );
        }

        let block_style: Style =
        if focused {
            self.config.theme_config.style_border_focused
        }
        else {
            self.config.theme_config.style_border_not_focused
        };

        let widget: Paragraph = 
        Paragraph::new(Line::from(spans))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(block_style)
                    .title(TITLE)
            );

        f.render_widget(widget, area);
        Ok(())
    }
}