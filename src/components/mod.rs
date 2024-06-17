use std::io;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;

pub mod system;

pub trait StatefulDrawableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> io::Result<bool>;
}

pub trait Component {
    fn event(&mut self, key: KeyEvent) -> io::Result<bool>;
}