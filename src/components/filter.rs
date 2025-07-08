use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode};
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

    pub fn input_str(&mut self) -> &str {
        &self.input_str
    }

    pub fn is_filter_empty(&mut self) -> bool {
        self.input_str.is_empty()
    }
}

impl Component for FilterComponent {
    fn event(&mut self, key: KeyEvent) -> Result<EventState> {
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
