use std::io;
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    prelude::*,
    widgets::*,
};

use crate::app::Focus;

use super::EventState;
use super::StatefulDrawableComponent;
use super::Component;

// Help acts as an observer of App.focus.
pub struct Help {
    help_text: String,
}

impl Help {
    pub fn new() -> Self {
        Self {
            help_text: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.help_text.clear();
    }

    pub fn update(&mut self, focus: Focus) {
        self.reset();
        match focus {
            Focus::ProcessList => {
                self.help_text = String::from("Press <Tab> to Search, <Down Arrow Key> to move focus down, <Up Arrow Key> to move focus up, <Q> to quit");
            }
            Focus::ProcessFilter => {
                self.help_text = String::from("Press <Tab> to navigate List, <Char(s)> to input name, <Enter> to filter, <Q> to quit");
            }
        }
    }
}

impl Component for Help {
    fn event(&mut self, _key: KeyEvent) -> io::Result<EventState> {
        return Ok(EventState::NotConsumed);
    }
}

impl StatefulDrawableComponent for Help {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> io::Result<()> {
        let widget: Paragraph = Paragraph::new(self.help_text.as_str())
            .style(Style::default().fg(Color::Green));
        f.render_widget(widget, area);
        return Ok(())
    }
}
