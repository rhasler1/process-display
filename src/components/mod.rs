use std::io;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;

pub mod system;
pub mod filter;
pub mod help;
pub mod cpu;
pub mod tab;
pub mod utils;
pub mod command;

pub trait StatefulDrawableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> io::Result<()>;
}

pub trait Component {
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState>;
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

#[derive(PartialEq, Clone)]
pub enum ListSortOrder {
    PidInc,
    PidDec,
    NameInc,
    NameDec,
    UsageInc,
    UsageDec,
}

impl Default for ListSortOrder {
    fn default() -> Self {
        ListSortOrder::UsageInc
    }
}