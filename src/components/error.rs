use anyhow::{Ok, Result};
use crate::input::*;
use ratatui::{
    Frame,
    prelude::*,
    widgets::*,
};
use crate::config::Config;
use crate::components::Component;
use super::{DrawableComponent, EventState};

pub struct ErrorComponent {
    pub error: String,
    visible: bool,
    config: Config,
}

impl ErrorComponent {
    pub fn new(config: Config) -> Self {
        Self {
            error: String::new(),
            visible: false,
            config,
        }
    }
}

impl ErrorComponent {
    pub fn set(&mut self, error: String) -> Result<()> {
        self.error = error;
        self.show()
    }

    fn hide(&mut self) -> Result<()> {
        self.visible = false;
        Ok(())
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;
        Ok(())
    }
}

impl Component for ErrorComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState> {
        if self.visible {
            if key == self.config.key_config.exit {
                self.error = String::new();
                self.hide()?;
                return Ok(EventState::Consumed);
            }
            return Ok(EventState::NotConsumed);
        }
        return Ok(EventState::NotConsumed)
    }

    fn mouse_event(&mut self, _mouse: Mouse) -> Result<EventState> {
        Ok(EventState::NotConsumed)
    }
}

impl DrawableComponent for ErrorComponent {
    fn draw(&mut self, f: &mut Frame, _area: Rect, _focused: bool) -> Result<()> {
        if self.visible {
            let width = 60;
            let height = 10;
            let error = Paragraph::new(self.error.to_string())
                .block(Block::default().title("Error").borders(Borders::ALL))
                .style(Style::default().fg(Color::Red))
                .wrap(Wrap { trim: true });

            let area = Rect::new(
                (f.size().width.saturating_sub(width)) / 2,
                (f.size().height.saturating_sub(height)) / 2,
                width.min(f.size().width),
                height.min(f.size().height),
            );
            f.render_widget(Clear, area);
            f.render_widget(error, area)
        }
        Ok(())
    }
}
