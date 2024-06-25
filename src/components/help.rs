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

//TODO: improve, currently not very helpful.
//
pub struct HelpComponent {
    help_text: String,
}

impl HelpComponent {
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
                self.help_text = String::from("Press <Tab> Filter, <Down Arrow Key> and <Up Arrow Key> navigate list, <T> terminate process, <ESC> reset System, <Q> quit application");
            }
            Focus::ProcessFilter => {
                self.help_text = String::from("Press <Tab> navigate List");
            }
        }
    }
}

impl Component for HelpComponent {
    fn event(&mut self, _key: KeyEvent) -> io::Result<EventState> {
        return Ok(EventState::NotConsumed);
    }
}

impl StatefulDrawableComponent for HelpComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> io::Result<()> {
        let widget: Paragraph = Paragraph::new(self.help_text.as_str())
            .style(Style::default().fg(Color::Green));
        f.render_widget(widget, area);
        return Ok(())
    }
}
