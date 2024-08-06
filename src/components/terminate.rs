use process_list::ProcessListItem;
use crate::config::KeyConfig;

use super::{Component, DrawableComponent};


pub struct TerminateComponent {
    visible: bool,
    key_config: KeyConfig,
    pid: u32,
    name: String,
}

impl TerminateComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            visible: false,
            key_config,
            pid: 0,
            name: String::new(),
        }
    }

    pub fn set_info(&mut self, item: Option<&ProcessListItem>) {
        if let Some(item) = item {
            self.pid = item.pid();
            self.name = item.name();
        }
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    fn show(&mut self) {
        self.visible = true;
    }
}

impl Component for TerminateComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> std::io::Result<super::EventState> {
        Ok(super::EventState::Consumed)
    }
}

impl DrawableComponent for TerminateComponent {
    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect, focused: bool) -> std::io::Result<()> {

        Ok(())
    }
}