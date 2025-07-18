use anyhow::{Ok, Result};
use crate::input::*;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};
use crate::config::Config;
use super::{EventState, DrawableComponent, Component};

#[derive(Default)]
pub struct FilterComponent {
    input_str: String,
    pub config: Config,
}

impl FilterComponent {
    pub fn new(config: Config) -> Self {
        Self {
            input_str: String::new(),
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
}

impl Component for FilterComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState> {
        match key {
            Key::Char(c) => {
                self.input_str.push(c);
                Ok(EventState::Consumed)
            }
            Key::Backspace => {
                self.input_str.pop();
                Ok(EventState::Consumed)
            }
            _ => Ok(EventState::NotConsumed)
        }
    }

    fn mouse_event(&mut self, _mouse: Mouse) -> Result<EventState> {
        Ok(EventState::NotConsumed)
    }
}

impl DrawableComponent for FilterComponent {
    fn draw(&mut self, f: &mut Frame, area: ratatui::prelude::Rect, focused: bool) -> Result<()> {
        let title: &str = " Filter ";

        let style: Style =
        if focused {
            self.config.theme_config.style_border_focused
        }
        else {
            self.config.theme_config.style_border_not_focused
        };

        let filter_text: &str = self.input_str.as_str();

        let widget: Paragraph =
            Paragraph::new(filter_text)
            .style(style)
            .block(Block::default().borders(Borders::ALL)
            .title(title));

        f.render_widget(widget, area);

        Ok(())
    }
}
