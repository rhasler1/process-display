use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use super::config::KeyConfig;
pub mod filter;
pub mod help;
pub mod error;
pub mod process;
pub mod utils;
pub mod command;
pub mod cpu;
pub mod memory;
pub mod temp;

pub trait DrawableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()>;
}

pub trait Component {
    fn event(&mut self, key: KeyEvent) -> Result<EventState>;
}

pub trait Refreshable<S> {
    fn refresh(&mut self, service: &S);
}

#[derive(PartialEq)]
pub enum EventState {
    Consumed,
    NotConsumed,
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }
}

pub fn common_nav(key: KeyEvent, key_config: &KeyConfig) -> Option<MoveSelection> {
    if key.code == key_config.move_down {
        Some(MoveSelection::Down)
    }
    else if key.code == key_config.move_bottom {
        Some(MoveSelection::Bottom)
    }
    else if key.code == key_config.move_up {
        Some(MoveSelection::Up)
    }
    else if key.code == key_config.move_top {
        Some(MoveSelection::Top)
    }
    else {
        None
    }
}

#[derive(Copy, Clone)]
pub enum MoveSelection {
    Up,
    Down,
    Top,
    Bottom,
}