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
}

pub struct App {
    pub focus: Focus, // the component the user is interacting with
    pub system_wrapper: SystemWrapper, // application component
    pub cpu: CPUComponent,
    pub config: Config, // key configuration
    pub tab: TabComponent,
}

impl App {
    // new constructor
    //
    pub fn new() -> Self {
        Self {
            focus: Focus::ProcessList,
            system_wrapper: SystemWrapper::new(),
            cpu: CPUComponent::default(),
            config: Config::default(),
            tab: TabComponent::new(),
        }
    }

    // pub async fn init -- initialize process
    //
    pub async fn init(&mut self) -> io::Result<()> {
        self.system_wrapper.refresh_all()?;
        let new_processes = self.system_wrapper.get_cpu_process_list();
        self.cpu.update(new_processes).await;
        Ok(())
    }

    // pub async fn update_process_list -- update on refresh events
    //
    pub async fn refresh(&mut self) -> io::Result<()> {
        // refresh the system_wrapper
        //
        self.system_wrapper.refresh_all()?;
        let new_processes = self.system_wrapper.get_cpu_process_list();

        // update the process list of selected tab
        match self.tab.selected_tab {
            Tab::CPU => {
                // refresh cpu list
                self.cpu.update(new_processes.as_ref()).await;
            }
            Tab::Memory => {}
        }
        Ok(())
    }

    // pub async func
    pub async fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if self.components_event(key).await?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        if self.move_focus(key).await?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        Ok(EventState::NotConsumed) // eventkey was not consumed
    }

    // async func
    async fn components_event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        // TODO: Implement error component for handling resetting the application on error

        // handle change tab (this is not affected by Focus variant)
        if self.tab.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        // match focus variants ProcessList and ProcessFilter
        match self.focus {
            Focus::ProcessList => {
                // match tab variants CPU and Memory
                match self.tab.selected_tab {
                    Tab::CPU => {
                        if self.cpu.event(key)?.is_consumed() {
                            return Ok(EventState::Consumed);
                        }
                    }
                    Tab::Memory => {
                        //TODO: implement MemoryComponent
                    }
                }
            }
        }
        return Ok(EventState::NotConsumed) // keyevent was not consumed
    }

    // move focus between the process list and filter
    async fn move_focus(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if key.code == self.config.key_config.tab {
            match self.focus {
                Focus::ProcessList => {
                    self.focus = Focus::ProcessList;
                }
            }
            return Ok(EventState::Consumed); // eventkey was consumed
        }
        return Ok(EventState::NotConsumed); // eventkey was not consumed
    }

    pub fn draw(&mut self, f: &mut Frame) -> io::Result<()> {
        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // tab bar
            Constraint::Min(1), // process list & filter
            Constraint::Length(3), // help bar
        ])
        .split(f.size());

        // draw tab component at top of frame
        self.tab.draw(f, chunks[0], false)?;
        // draw filter component below tab component
        // draw process list in the largest area
        match self.tab.selected_tab {
            Tab::CPU => {
                self.cpu.draw(f, chunks[1], false)?;
            }
            Tab::Memory => {
                //self.memory.draw(f, chunks[1])?;
            }
        }
        // draw help bar at the bottom of the frame
        return Ok(())
    }
}