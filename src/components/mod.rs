use std::io;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;

pub mod system;
pub mod filter;
pub mod help;
pub mod error;
pub mod process;
pub mod terminate;
pub mod performance;
pub mod tab;
pub mod utils;
pub mod command;

pub trait DrawableComponent {
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