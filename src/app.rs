use std::io::{self};
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use crate::config::Config;
use crate::components::{
    tab::TabComponent,
    cpu::CPUComponent,
    system::SystemWrapper,
    Component,
    EventState,
    StatefulDrawableComponent,
    tab::Tab,
    help::HelpComponent,
    command,
    command::CommandInfo,
};

#[derive(Clone, Copy)]
pub enum Focus {
    ProcessList,
}

pub struct App {
    _focus: Focus,
    system_wrapper: SystemWrapper,
    cpu: CPUComponent,
    tab: TabComponent,
    help: HelpComponent,
    pub config: Config,
}

impl App {
    // new constructor
    //
    pub fn new(config: Config) -> Self {
        Self {
            _focus: Focus::ProcessList,
            system_wrapper: SystemWrapper::new(),
            cpu: CPUComponent::default(),
            tab: TabComponent::new(),
            help: HelpComponent::new(config.key_config.clone()),
            config: config.clone(),
        }
    }

    // pub async function to process KeyEvent's
    //
    pub async fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        // 1. update commands
        self.update_commands();

        // 2. check if key is consumed by a component
        if self.components_event(key).await?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        // 3. else check if key is consumed by moving focus of application
        // note: Currently, the application struct only has one focus
        // state, as a result the move_focus() function does not do anything.
        else if self.move_focus(key).await?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        // 4. else key is not consumed
        Ok(EventState::NotConsumed)
    }

    // function to populate the help component with CommandInfo
    //
    fn update_commands(&mut self) {
        self.help.set_commands(self.commands());
    }

    // function to populate a Vector with CommandInfo
    //
    fn commands(&self) -> Vec<CommandInfo> {
        let res = vec![
            CommandInfo::new(command::help(&self.config.key_config)),
            CommandInfo::new(command::exit_popup(&self.config.key_config)),
            CommandInfo::new(command::change_tab(&self.config.key_config)),
            CommandInfo::new(command::move_selection(&self.config.key_config)),
            CommandInfo::new(command::selection_to_top_bottom(&self.config.key_config)),
            CommandInfo::new(command::follow_selection(&self.config.key_config)),
            CommandInfo::new(command::sort_list_by_name(&self.config.key_config)),
            CommandInfo::new(command::sort_list_by_pid(&self.config.key_config)),
            CommandInfo::new(command::sort_list_by_usage(&self.config.key_config)),
            CommandInfo::new(command::filter_submit(&self.config.key_config)),
            CommandInfo::new(command::terminate_process(&self.config.key_config)),
        ];
        res
    }

    // async function to process component's event
    //
    async fn components_event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        // TODO: Implement error component for handling resetting the application on error

        // 1. check if help component can process key event
        if self.help.event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        // 2. check if tab component can process key event
        if self.tab.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        // 3. check if the selected tab component can process key event
        match self.tab.selected_tab {
            Tab::CPU => {
                if self.cpu.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed);
                }
            }

            Tab::Memory => { // todo: implement MemoryComponent
                //if self.memory.event(key)?.is_consumed() {
                //    return Ok(EventState::Consumed)
                //}
            }
        }

        // 4. else key event is not consumed
        return Ok(EventState::NotConsumed)
    }

    // async function to move the focus of the application
    // currently, the function does not do anything, keeping as
    // placeholder for adding new features.
    //
    async fn move_focus(&mut self, _key: KeyEvent) -> io::Result<EventState> {
        return Ok(EventState::NotConsumed); // eventkey was not consumed
    }

    // pub async fn update_process_list -- update on refresh events
    //
    pub async fn refresh(&mut self) -> io::Result<()> {
        // 1. refresh system
        self.system_wrapper.refresh_all()?;
        // 2. get new process list
        let new_processes = self.system_wrapper.get_cpu_process_list();
        // 3. update the process list of the selected tab
        match self.tab.selected_tab {
            Tab::CPU => {
                self.cpu.update(new_processes.as_ref()).await?;
            }

            Tab::Memory => { //todo
                //self.memory.update(new_processes.as_ref()).await?;
            }
        }
        
        Ok(())
    }

    pub fn draw(&mut self, f: &mut Frame) -> io::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1), // process list & filter
                //Constraint::Length(3),
            ])
            .split(f.size());

        //
        //self.title.draw()
        
        // draw tab component at top of frame
        //
        self.tab.draw(f, chunks[0], false)?;

        // draw selected tab component in the largest chunk
        //
        match self.tab.selected_tab {
            Tab::CPU => {
                self.cpu.draw(f, chunks[0], false)?;
            }
            Tab::Memory => {
                //self.memory.draw(f, chunks[1])?;
            }
        }

        // draw help as pop up
        //
        self.help.draw(f, Rect::default(), false)?;

        
        return Ok(())
    }
}