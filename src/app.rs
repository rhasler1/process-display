use anyhow::{Ok, Result};
use crossterm::event::{KeyEvent};
use ratatui::prelude::*;
use crate::config::Config;
use crate::components::{
    cpu::CPUComponent,
    process::ProcessComponent,
    sysinfo_wrapper::SysInfoWrapper,
    error::ErrorComponent,
    Component,
    EventState,
    DrawableComponent,
    help::HelpComponent,
    command,
    command::CommandInfo,
};

enum MainFocus {
    CPU,
    Process,
}

pub struct App {
    focus: MainFocus,
    expand: bool,
    system_wrapper: SysInfoWrapper,
    process: ProcessComponent,
    cpu: CPUComponent,
    help: HelpComponent,
    pub error: ErrorComponent,
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        let mut system_wrapper = SysInfoWrapper::new(config.clone());
        
        system_wrapper.refresh_all();
        
        let processes = system_wrapper.get_processes();

        let mut cpu = CPUComponent::default();

        let cpus = system_wrapper.get_cpus();

        cpu.update(cpus);

        Self {
            focus: MainFocus::Process,
            expand: false,
            system_wrapper,
            process: ProcessComponent::new(config.clone(), processes),
            cpu,
            help: HelpComponent::new(config.clone()),
            error: ErrorComponent::new(config.clone()),
            config: config.clone(),
        }
    }

    pub fn refresh_event(&mut self) -> Result<EventState> {
        self.system_wrapper.refresh_all();

        self.update_process();

        self.update_cpu();

        self.update_cmds();

        Ok(EventState::Consumed)
    }

    fn update_process(&mut self) {
        let new_processes = self.system_wrapper.get_processes();    // receive ownership
        
        self.process.update(new_processes);                         // transfer ownership
    }

    fn update_cpu(&mut self) {
        let new_cpus = self.system_wrapper.get_cpus();  // receive ownership

        self.cpu.update(new_cpus);                      // transfer ownership
    }

    fn toggle_expand(&mut self) {
        self.expand = !self.expand
    }

    pub fn key_event(&mut self, key: KeyEvent) -> Result<EventState> {
        if self.component_event(key)?.is_consumed() {
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

    fn component_event(&mut self, key: KeyEvent) -> Result<EventState> {
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
            MainFocus::Process => {
                if self.process.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
        }

        Ok(EventState::NotConsumed)
    }

    fn move_focus(&mut self, key: KeyEvent) -> Result<EventState> {
        if key.code == self.config.key_config.tab {
            match self.focus {
                MainFocus::CPU => {
                    self.focus = MainFocus::Process
                }
                MainFocus::Process => {
                    self.focus = MainFocus::CPU
                }
            }

            return Ok(EventState::Consumed)
        }

        Ok(EventState::NotConsumed)
    }

    pub fn draw(&mut self, f: &mut Frame) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
            ])
            .split(f.size());

        self.error.draw(f, chunks[0], false)?;


        if self.expand {
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
        }
        else {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(75),
                ].as_ref())
                .split(chunks[0]);
            
            let mut horizontal_chunks = Vec::new();

            for chunk in vertical_chunks.iter() {
                let horizontal_chunk = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(34),
                    ])
                    .split(*chunk);

                horizontal_chunks.push(horizontal_chunk);
            }

            self.process.draw(
                f,
                vertical_chunks[1],
                matches!(self.focus, MainFocus::Process)
            )?;

            self.cpu.draw(
                f,
                vertical_chunks[0],
                matches!(self.focus, MainFocus::CPU)
            )?;
        }

        self.help.draw(f, Rect::default(), false)?;

        return Ok(())
    }

    fn update_cmds(&mut self) {
        let cmds = self.commands();

        self.help.set_commands(cmds);
    }

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
}

