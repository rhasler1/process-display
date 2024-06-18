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
    search_bar::SearchBar,
    help::Help,
    StatefulDrawableComponent,
    Component,
};

#[derive(Clone, Copy)]
pub enum Focus {
    //TODO: implement
    System,
    SearchBar,
}

pub struct App {
    pub focus: Focus, // the component the user is interacting with
    pub system_wrapper: SystemWrapper, // application component
    pub search_bar: SearchBar, // application component
    pub help: Help, // observer of Focus
}

impl App {
    // default
    pub fn new() -> Self {
        Self {
            focus: Focus::System,
            system_wrapper: SystemWrapper::new(),
            search_bar: SearchBar::new(),
            help: Help::new(),
        }
    }

    pub fn reset(&mut self) {
        self.focus = Focus::System;
        self.system_wrapper.reset();
        self.search_bar.reset();
        self.help.update(self.focus); // observer of app.focus (app state)
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
                if self.search_bar.is_empty() && self.system_wrapper.event(key, None)? {
                    return Ok(true) // keyevent was consumed
                }
                else if !self.search_bar.is_empty() && self.system_wrapper.event(key, Some(self.search_bar.get_process_name()))? {
                    return Ok(true) // keyevent was consumed
                }
            }
            Focus::SearchBar => {
                if self.search_bar.event(key, None)? {return Ok(true) } // keyevent was consumed
            }
        }
        return Ok(false) // keyevent was not consumed
    }

    // TAB -> used to change focus b/w System and SearchBar
    fn move_focus(&mut self, key: KeyEvent) -> io::Result<bool> {
        if key.code == KeyCode::Tab {
            match self.focus {
                Focus::System => {
                    self.focus = Focus::SearchBar;
                }
                Focus::SearchBar => {
                    self.focus = Focus::System;
                }
            }
            return Ok(true); // eventkey was consumed
        }
        return Ok(false); // eventkey was not consumed
    }

    pub fn draw(&mut self, f: &mut Frame) -> io::Result<bool> {
        // update help component (observer) before drawing
        self.help.update(self.focus);

        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search Bar
            Constraint::Min(1), // System Wrapper (Process List)
            Constraint::Length(3), // Help Bar
        ])
        .split(f.size());

        self.search_bar.draw(f, chunks[0])?;
        self.system_wrapper.draw(f, chunks[1])?;
        self.help.draw(f, chunks[2])?;

        return Ok(true)
    }
}