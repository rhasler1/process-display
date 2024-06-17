use std::io::{self};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    event::{self, EnableMouseCapture, Event, KeyEvent, DisableMouseCapture, KeyCode}
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    prelude::*
};

use crate::components::{
    system::SystemWrapper,
    StatefulDrawableComponent,
    Component,
};

pub enum Focus {
    //TODO: implement
    System,
}

pub struct App {
    pub focus: Focus,
    pub system_wrapper: SystemWrapper,
}

impl App {
    pub fn new() -> Self {
        Self {
            focus: Focus::System,
            system_wrapper: SystemWrapper::new(),
        }
    }

    pub fn reset(&mut self) -> io::Result<bool> {
        self.system_wrapper.reset()?;
        return Ok(true)
    }

    pub fn event(&mut self, key: KeyEvent) -> io::Result<bool> {
        if self.components_event(key)? {
            return Ok(true);
        }
        if self.move_focus(key)? {
            return Ok(true);
        }
        Ok(false) // eventkey was not consumed
    }

    fn components_event(&mut self, key: KeyEvent) -> io::Result<bool> {
        //TODO: implement
        match self.focus {
            Focus::System => {
                self.system_wrapper.event(key)?;
            }
        }
        return Ok(true)
    }

    // currently does not do anything, only one focus is supported at the application level
    fn move_focus(&mut self, _key: KeyEvent) -> io::Result<bool> {
        match self.focus {
            Focus::System => {
                self.focus = Focus::System;
            }
        }
        return Ok(true)
    }

    pub fn draw(&mut self, f: &mut Frame) -> io::Result<bool> {
        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(1), // Component
            Constraint::Length(3), // Help
        ])
        .split(f.size());

        match self.focus {
            Focus::System => {
                self.system_wrapper.draw(f, chunks[1])?;
            }
        }

        return Ok(true)
    }
}