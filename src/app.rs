use anyhow::{Ok, Result};
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use crate::components::cpu::CPUComponent;
use crate::config::Config;
use crate::components::{
    tab::TabComponent,
    process::ProcessComponent,
    system_wrapper::SystemWrapper,
    system_component::SystemComponent,
    error::ErrorComponent,
    Component,
    EventState,
    DrawableComponent,
    help::HelpComponent,
    command,
    command::CommandInfo,
};

pub enum MainFocus {
    CPU,
    System,
    Memory,
    Network,
    Temperature,
    Process,
}

pub struct App {
    focus: MainFocus,
    expand: bool,
    system_wrapper: SystemWrapper,
    system_component: SystemComponent,
    process: ProcessComponent,
    cpu: CPUComponent,
    tab: TabComponent,
    help: HelpComponent,
    pub error: ErrorComponent,
    pub config: Config,
}

impl App {
    // constructor.
    pub fn new(config: Config) -> Self {
        Self {
            focus: MainFocus::Process,
            expand: false,
            system_wrapper: SystemWrapper::new(config.clone()),
            system_component: SystemComponent::default(),
            process: ProcessComponent::new(config.clone()),
            cpu: CPUComponent::default(),
            tab: TabComponent::new(config.clone()),
            help: HelpComponent::new(config.clone()),
            error: ErrorComponent::new(config.clone()),
            config: config.clone(),
        }
    }

    // call after constructor
    pub fn init(&mut self) -> Result<()> {
        self.system_wrapper.refresh_all()?;
        self.init_system_component();
        self.update_process();
        self.update_cpu();
        //self.update_performance()?;
        self.update_commands();

        Ok(())
    }

    fn init_system_component(&mut self) {
        let vec: Vec<String> = SystemWrapper::get_static_sysinfo();
        self.system_component.init(vec); // transfer ownership
    }

    // refresh system and dependencies
    pub fn refresh(&mut self) -> Result<()>{
        self.system_wrapper.refresh_all()?;
        self.update_process();
        self.update_cpu();
        //self.update_performance()?;

        Ok(())
    }

    fn update_cpu(&mut self) -> bool {
        let new_cpus = self.system_wrapper.get_cpus();
        self.cpu.update(&new_cpus);
        true
    }

    // return result of process update
    fn update_process(&mut self) -> bool {
        let new_processes = self.system_wrapper.get_processes();
        let res = self.process.update(&new_processes);

        res
    }

    // fix return type
    //fn update_performance(&mut self) -> Result<()> {
    //    let new_cpu_info = self.system.get_cpu_info();
    //    let new_memory_info = self.system.get_memory_info();
    //    self.performance.update(&new_cpu_info, &new_memory_info)?;

    //   Ok(())
    //}

    fn toggle_expand(&mut self) {
        if self.expand == true {
            self.expand = false;
        }
        else {
            self.expand = true;
        }
    }

    // top level key event processor
    pub fn event(&mut self, key: KeyEvent) -> Result<EventState> {
        //if key.code == self.config.key_config.toggle_themes {
        //    self.update_component_themes();
        //    return Ok(EventState::Consumed)
        //}
        if self.components_event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        else if self.move_focus(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        else if key.code == self.config.key_config.expand {
            self.toggle_expand();
            return Ok(EventState::Consumed);
        }

        Ok(EventState::NotConsumed)
    }

    // toggle color scheme
    //fn update_component_themes(&mut self) {
    //    self.config.theme_config.toggle_themes();
    //    self.process.config.theme_config.toggle_themes();
    //    //self.performance.config.theme_config.toggle_themes();
    //    self.help.config.theme_config.toggle_themes();
    //    self.system._config.theme_config.toggle_themes();
    //    self.tab.config.theme_config.toggle_themes();
    //}
    
    // update help dialogue--commands
    fn update_commands(&mut self) {
        self.help.set_commands(self.commands());
    }

    // set commands
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
            CommandInfo::new(command::sort_list_by_cpu_usage(&self.config.key_config)),
            CommandInfo::new(command::sort_list_by_memory_usage(&self.config.key_config)),
            CommandInfo::new(command::filter_submit(&self.config.key_config)),
            CommandInfo::new(command::terminate_process(&self.config.key_config)),
        ];

        res
    }

    /* */

    // component key event processor
    fn components_event(&mut self, key: KeyEvent) -> Result<EventState> {
        if self.error.event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        if self.help.event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        match self.focus {
            MainFocus::CPU => {
                if self.cpu.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::System => {
                if self.system_component.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Memory => {}
            MainFocus::Network => {}
            MainFocus::Temperature => {}
            MainFocus::Process => {
                if self.process.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
        }

        if self.tab.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        
        Ok(EventState::NotConsumed)
    }


    /*
    Control with Tab
    CPU -> Memory -> Network -> Temperature -> Process -> ...
     */
    fn move_focus(&mut self, key: KeyEvent) -> Result<EventState> {
        if key.code == self.config.key_config.tab {
            match self.focus {
                MainFocus::CPU => {
                    self.focus = MainFocus::Memory
                }
                MainFocus::Memory => {
                    self.focus = MainFocus::Network
                }
                MainFocus::Network => {
                    self.focus = MainFocus::Temperature
                }
                MainFocus::Temperature => {
                    self.focus = MainFocus::Process
                }
                MainFocus::Process => {
                    self.focus = MainFocus::System
                }
                MainFocus::System => {
                    self.focus = MainFocus::CPU
                }
            }
            return Ok(EventState::Consumed)
        }
        Ok(EventState::NotConsumed)
    }

    // draw the app
    pub fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
            ])
            .split(f.size());

        // if error always draw--error component state determines if anything is drawn
        self.error.draw(f, chunks[0], false)?;


        if self.expand {
            // split screen to draw only focused component
            if matches!(self.focus, MainFocus::Process) {
                self.process.draw(
                    f,
                    chunks[0],
                    true,
                )?;
            }

            if matches!(self.focus, MainFocus::CPU) {
                self.cpu.draw(
                    f,
                    chunks[0],
                    true,
                )?;
            }

            if matches!(self.focus, MainFocus::System) {
                self.system_component.draw(
                    f,
                    chunks[0],
                    true,
                )?;
            }
        }
        else {
            // draw all components
            // split screen
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ].as_ref())
                .split(chunks[0]);
            
            let mut horizontal_chunks = Vec::new();
            for chunk in vertical_chunks.iter() {
                let horizontal_chunk = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Fill(1),
                        Constraint::Percentage(80), 
                    ])
                    .split(*chunk);

                horizontal_chunks.push(horizontal_chunk);
            }

            self.process.draw(
                f,
                horizontal_chunks[3][1],
                matches!(self.focus, MainFocus::Process)
            )?;

            self.system_component.draw(
                f,
                horizontal_chunks[3][0],
                matches!(self.focus, MainFocus::System)
            )?;

            self.cpu.draw(
                f,
                vertical_chunks[0],
                matches!(self.focus, MainFocus::CPU)
            )?;
        }

        // if help--help component state determines if anything is drawn
        self.help.draw(f, Rect::default(), false)?;

        return Ok(())
    }
}