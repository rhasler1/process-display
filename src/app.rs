use std::io::{self};
use crossterm::event::KeyEvent;
use ratatui::prelude::*;

use crate::config::Config;
use crate::components::{
    tab::TabComponent,
    help::HelpComponent,
    filter::FilterComponent,
    cpu::CPUComponent,
    system::SystemWrapper,
    Component,
    EventState,
    StatefulDrawableComponent,
    tab::Tab,
    Action,
};

#[derive(Clone, Copy)]
pub enum Focus {
    ProcessList,
    ProcessFilter,
}

pub struct App {
    pub action: Option<Action>,
    pub focus: Focus, // the component the user is interacting with
    pub system_wrapper: SystemWrapper, // application component
    pub cpu: CPUComponent,
    pub process_filter: FilterComponent, // application component
    pub help: HelpComponent, // observer of Focus
    pub config: Config, // key configuration
    pub tab: TabComponent,
}

impl App {
    // default
    pub fn new() -> Self {
        Self {
            action: None,
            focus: Focus::ProcessList,
            system_wrapper: SystemWrapper::new(),
            cpu: CPUComponent::new(),
            process_filter: FilterComponent::new(),
            help: HelpComponent::new(),
            config: Config::default(),
            tab: TabComponent::new(),
        }
    }

    pub fn reset(&mut self) {
        self.action = None;
        self.focus = Focus::ProcessList;
        self.system_wrapper.reset();
        self.cpu.reset();
        self.cpu.set_unfiltered_list(self.system_wrapper.get_process_list());
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
        // handle reset event-- full app reset
        if key.code == self.config.key_config.reset {
            self.reset();
            return Ok(EventState::Consumed);
        }

        // handle change tab (this is not affected by Focus variant)
        if self.tab.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        // match focus variants ProcessList and ProcessFilter
        match self.focus {
            Focus::ProcessList => {
                // On events terminate, suspend, and resume the SystemWrapper and observers, ie: CPU and memory need to be updated,
                // this is handled in the app.update() method, which executes in the main event loop after
                // app.event(key).
                if key.code == self.config.key_config.terminate {
                    self.action = Some(Action::Terminate);
                    return Ok(EventState::Consumed);
                }
                if key.code == self.config.key_config.suspend {
                    self.action = Some(Action::Suspend);
                    return Ok(EventState::Consumed);
                }
                if key.code == self.config.key_config.resume {
                    self.action = Some(Action::Resume);
                    return Ok(EventState::Consumed);
                }
                // match tab variants CPU and Memory
                match self.tab.selected_tab {
                    Tab::CPU => {
                        if self.cpu.event(key)?.is_consumed() {
                            return Ok(EventState::Consumed);
                        }
                    }
                    Tab::Memory => {
                        //TODO: implement MemoryComponent
                        //if self.memory.event(key)?.is_consumed() { // TODO: implement memory struct
                        //    return Ok(EventState::Consumed);
                        //}
                    }
                }
            }
            Focus::ProcessFilter => {
                if self.process_filter.event(key)?.is_consumed() {
                    self.action = Some(Action::Filtering);
                    return Ok(EventState::Consumed); // keyevent was consumed
                }
            }
        }
        self.action = None; // if keyevent was not consumed, set action to None
        return Ok(EventState::NotConsumed) // keyevent was not consumed
    }

    // move focus between the process list and filter
    fn move_focus(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if key.code == self.config.key_config.tab {
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

    pub fn update(&mut self) -> io::Result<()> {
        // update all components w/ action
        if self.action.is_some() {
            let action = self.action.unwrap();
            match action {
                // set the filtered list in cpu/memory with filter information
                Action::Filtering => {
                    match self.tab.selected_tab {
                        Tab::CPU => {
                            self.cpu.set_filtered_list(self.process_filter.get_filter());
                        }
                        Tab::Memory => {
                            //self.memory.set_filter_name(self.process_filter.get_filter()); //TODO: IMPLEMENT
                        }
                    }
                }
                // terminate the pid in focus and reset application
                Action::Terminate => {
                    match self.tab.selected_tab {
                        Tab::CPU => {
                            if self.cpu.get_pid().is_some() {
                                let pid = self.cpu.get_pid().unwrap();
                                self.system_wrapper.terminate_process(pid)?;
                                self.reset(); // resetting the entire app to deal with termination, TODO: improve.
                            }
                        }
                        Tab::Memory => {
                            //TODO: implement MemoryComponent
                        }
                    }
                }
                Action::Resume => {}//TODO
                Action::Suspend => {}//TODO
            }
        }
        // updating the help component before drawing
        self.help.update(self.focus);
        return Ok(())
    }

    pub fn draw(&mut self, f: &mut Frame) -> io::Result<()> {
        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // tab bar
            Constraint::Length(3), // filter bar
            Constraint::Min(1), // process list
            Constraint::Length(3), // help bar
        ])
        .split(f.size());

        // draw tab component at top of frame
        self.tab.draw(f, chunks[0])?;
        // draw filter component below tab component
        self.process_filter.draw(f, chunks[1])?;
        // draw process list in the largest area
        match self.tab.selected_tab {
            Tab::CPU => {
                self.cpu.draw(f, chunks[2])?;
            }
            Tab::Memory => {
                //self.memory.draw(f, chunks[1])?;
            }
        }
        // draw help bar at the bottom of the frame
        self.help.draw(f, chunks[3])?;
        return Ok(())
    }
}