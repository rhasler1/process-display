use std::io;
use crossterm::event::KeyEvent;

use ratatui::prelude::*;

use crate::config::KeyConfig;
use crate::process::process_list::MoveSelection;

pub mod system;
pub mod filter;
pub mod help;
pub mod cpu;
pub mod tab;
pub mod utils;

pub trait StatefulDrawableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> io::Result<()>;
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

#[derive(PartialEq, Clone)]
pub enum ListSortOrder {
    PidInc,
    PidDec,
    NameInc,
    NameDec,
    UsageInc,
    UsageDec,
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }
}