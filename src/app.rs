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
    process_list::ProcessList,
    process_filter::ProcessFilter,
    help::Help,
    StatefulDrawableComponent,
    Component,
};

#[derive(Clone, Copy)]
pub enum Focus {
    //TODO: implement
    ProcessList,
    ProcessFilter,
}

pub struct App {
    pub focus: Focus, // the component the user is interacting with
    pub system_wrapper: SystemWrapper, // application component
    pub process_list: ProcessList,
    pub process_filter: ProcessFilter, // application component
    pub help: Help, // observer of Focus
}

impl App {
    // default
    pub fn new() -> Self {
        Self {
            focus: Focus::ProcessList,
            system_wrapper: SystemWrapper::new(),
            process_list: ProcessList::new(),
            process_filter: ProcessFilter::new(),
            help: Help::new(),
        }
    }

    pub fn reset(&mut self) {
        self.focus = Focus::ProcessList;
        self.system_wrapper.reset();
        self.process_list.reset();
        self.process_list.set_unfiltered_list(self.system_wrapper.get_process_list());
        self.process_filter.reset();
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
        if key.code == KeyCode::Esc {
            self.reset();
            return Ok(true)
        }
        match self.focus {
            Focus::ProcessList => {
                if self.process_list.event(key)? {
                    return Ok(true)
                }
                if key.code == KeyCode::Char('t') {
                    // terminate process
                    self.system_wrapper.terminate_process(self.process_list.get_pid())?;
                    // update process_list in system_wrapper.
                    self.system_wrapper.reset();
                    // update unfiltered_list in process_list.
                    self.process_list.set_unfiltered_list(self.system_wrapper.get_process_list());
                }
            }
            Focus::ProcessFilter => {
                if self.process_filter.event(key)? {
                    return Ok(true); // keyevent was consumed
                }
                if key.code == KeyCode::Enter {
                    // set process_list.filter_name
                    self.process_list.set_filter_name(self.process_filter.get_filter_name());
                    // set process_list.filtered_list
                    self.process_list.set_filtered_list();
                    return Ok(true);
                }
            }
        }
        return Ok(false) // keyevent was not consumed
    }

    // TAB -> used to change focus b/w System and SearchBar
    fn move_focus(&mut self, key: KeyEvent) -> io::Result<bool> {
        if key.code == KeyCode::Tab {
            match self.focus {
                Focus::ProcessList => {
                    self.focus = Focus::ProcessFilter;
                }
                Focus::ProcessFilter => {
                    self.focus = Focus::ProcessList;
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

        self.process_filter.draw(f, chunks[0])?;
        self.process_list.draw(f, chunks[1])?;
        self.help.draw(f, chunks[2])?;

        return Ok(true)
    }
}