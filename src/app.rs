use std::io::{self};
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::prelude::*;

use crate::config::Config;
use crate::components::{
    help::Help,
    process_filter::ProcessFilter,
    process_list::ProcessList,
    system::SystemWrapper,
    Component,
    EventState,
    StatefulDrawableComponent
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
    pub config: Config, // key configuration
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
            config: Config::default(),
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

    pub fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if self.components_event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        if self.move_focus(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        Ok(EventState::NotConsumed) // eventkey was not consumed
    }

    fn components_event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if key.code == KeyCode::Esc {
            self.reset();
            return Ok(EventState::Consumed)
        }
        match self.focus {
            Focus::ProcessList => {
                if self.process_list.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
                if key.code == KeyCode::Char('t') {
                    // terminate process
                    if self.process_list.get_pid().is_some() {
                        // pass the pid in focus in process_list to system_wrapper.terminate_process().
                        self.system_wrapper.terminate_process(self.process_list.get_pid().unwrap())?;
                        // reset app
                        self.reset();

                    }
                    return Ok(EventState::Consumed)
                }
            }
            Focus::ProcessFilter => {
                if self.process_filter.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed); // keyevent was consumed
                }
                if key.code == KeyCode::Enter {
                    // set process_list.filter_name
                    self.process_list.set_filter_name(self.process_filter.get_filter_name());
                    // set process_list.filtered_list
                    self.process_list.set_filtered_list();
                    return Ok(EventState::Consumed);
                }
            }
        }
        return Ok(EventState::NotConsumed) // keyevent was not consumed
    }

    // TAB -> used to change focus b/w System and SearchBar
    fn move_focus(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if key.code == KeyCode::Tab {
            match self.focus {
                Focus::ProcessList => {
                    self.focus = Focus::ProcessFilter;
                }
                Focus::ProcessFilter => {
                    self.focus = Focus::ProcessList;
                }
            }
            return Ok(EventState::Consumed); // eventkey was consumed
        }
        return Ok(EventState::NotConsumed); // eventkey was not consumed
    }

    pub fn draw(&mut self, f: &mut Frame) -> io::Result<()> {
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

        return Ok(())
    }
}