use std::io;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;

pub mod system;
pub mod filter;
pub mod help;
pub mod cpu;
pub mod tab;

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
