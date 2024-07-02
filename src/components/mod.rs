use std::io;
use crossterm::event::KeyEvent;
use process_list::MoveSelection;
use ratatui::prelude::*;

use crate::config::KeyConfig;

pub mod system;
pub mod filter;
pub mod help;
pub mod cpu;
pub mod tab;
pub mod process_list_items;
pub mod list_items_iter;
pub mod process_list;
pub mod list_iter;

pub trait StatefulDrawableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> io::Result<()>;
}

pub trait Component {
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState>;
}

#[derive(Clone, Copy)]
pub enum Action {
    Terminate,
    Suspend,
    Resume,
    Filtering,
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
        Some(MoveSelection::End)
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