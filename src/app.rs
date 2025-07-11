use anyhow::{Ok, Result};
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use crate::components::temp::TempComponent;
use crate::config::Config;
use crate::components::{
    cpu::CPUComponent,
    memory::MemoryComponent,
    process::ProcessComponent,
    sysinfo_wrapper::SysInfoWrapper,
    error::ErrorComponent,
    Component,
    EventState,
    DrawableComponent,
    help::HelpComponent,
};

enum MainFocus {
    CPU,
    Process,
    Memory,
    Temp,
}

pub struct App {
    focus: MainFocus,
    expand: bool,
    system_wrapper: SysInfoWrapper,
    process: ProcessComponent,
    cpu: CPUComponent,
    memory: MemoryComponent,
    temp: TempComponent,
    help: HelpComponent,
    pub error: ErrorComponent,
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        let mut system_wrapper = SysInfoWrapper::new(config.clone());
        system_wrapper.refresh_all();
        
        let process = ProcessComponent::new(config.clone(), &system_wrapper);
        let memory = MemoryComponent::new(config.clone(), &system_wrapper);
        let cpu = CPUComponent::new(config.clone(), &system_wrapper);
        let temp = TempComponent::new(config.clone(), &system_wrapper);

        Self {
            focus: MainFocus::Process,
            expand: false,
            system_wrapper,
            process,
            cpu,
            memory,
            temp,
            help: HelpComponent::new(config.clone()),
            error: ErrorComponent::new(config.clone()),
            config: config.clone(),
        }
    }

    pub fn refresh_event(&mut self) -> Result<EventState> {
        self.system_wrapper.refresh_all();

        self.process.update(&self.system_wrapper);
        self.memory.update(&self.system_wrapper);
        self.cpu.update(&self.system_wrapper);
        self.temp.update(&self.system_wrapper);

        Ok(EventState::Consumed)
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
            MainFocus::Memory => {
                if self.memory.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Temp => {
                if self.temp.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Process => {
                if self.process.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
                // terminate case
                if key.code == self.config.key_config.terminate {
                    self.process.terminate_process(&self.system_wrapper);

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
                    self.focus = MainFocus::Memory
                }
                MainFocus::Memory => {
                    self.focus = MainFocus::Temp
                }
                MainFocus::Temp => {
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

            if matches!(self.focus, MainFocus::Memory) {
                self.memory.draw(
                    f,
                    chunks[0],
                    true,
                )?;
            }

            if matches!(self.focus, MainFocus::Temp) {
                self.temp.draw(
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
                    Constraint::Percentage(24),
                    Constraint::Percentage(24),
                    Constraint::Percentage(52),
                ].as_ref())
                .split(chunks[0]);
            
            let mut horizontal_chunks = Vec::new();

            for chunk in vertical_chunks.iter() {
                let horizontal_chunk = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(*chunk);

                horizontal_chunks.push(horizontal_chunk);
            }

            self.process.draw(
                f,
                vertical_chunks[2],
                matches!(self.focus, MainFocus::Process)
            )?;

            self.cpu.draw(
                f,
                vertical_chunks[0],
                matches!(self.focus, MainFocus::CPU)
            )?;

            self.memory.draw(
                f,
                horizontal_chunks[1][0],
                //vertical_chunks[1],
                matches!(self.focus, MainFocus::Memory)
            )?;

            self.temp.draw(
                f,
                horizontal_chunks[1][1],
                matches!(self.focus, MainFocus::Temp)
            )?;
        }

        self.help.draw(f, Rect::default(), false)?;

        return Ok(())
    }

    /*
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
    }*/
}
