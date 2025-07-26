use anyhow::Result;
use crate::input::{Key, Mouse};
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
pub mod network;

pub trait DrawableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()>;
}

pub trait Component {
    fn key_event(&mut self, key: Key) -> Result<EventState>;
    fn mouse_event(&mut self, mouse: Mouse) -> Result<EventState>;
}

// trait Refreshable details:
//
// Refreshable is meant to be implemented in components that are refreshable
// via a service (e.g., components/process.rs). Currently, there is only
// one service available and can be found in services/sysinfo_service.rs--
// this is essentially just a wrapper around the sysinfo crate.
// For more information on what sysinfo service provides to components,
// see trait VecProvider<T> in services/mod.rs.
//
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

pub fn common_nav(key: Key, key_config: &KeyConfig) -> Option<MoveSelection> {
    if key == key_config.move_down {
        Some(MoveSelection::Down)
    }
    else if key == key_config.move_bottom {
        Some(MoveSelection::Bottom)
    }
    else if key == key_config.move_up {
        Some(MoveSelection::Up)
    }
    else if key == key_config.move_top {
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