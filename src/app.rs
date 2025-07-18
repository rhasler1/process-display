use std::collections::HashMap;
use anyhow::{Ok, Result};
use crossterm::event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::prelude::*;
use crate::input::{Key, Mouse, MouseKind};
use crate::components::temp::TempComponent;
use crate::components::Refreshable;
use crate::config::Config;
use crate::components::{
    cpu::CPUComponent,
    memory::MemoryComponent,
    process::ProcessComponent,
    error::ErrorComponent,
    EventState,
    Component,
    DrawableComponent,
    help::HelpComponent,
};
use crate::services::sysinfo_service::SysInfoService;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum MainFocus {
    CPU,
    Process,
    Memory,
    Temp,
}

pub struct App {
    focus: MainFocus,
    focus_rects: HashMap<MainFocus, Rect>,
    expand: bool,
    service: SysInfoService,
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
        let mut service = SysInfoService::new(config.clone());
        service.refresh_all();
        
        let process = ProcessComponent::new(config.clone(), &service);
        let memory = MemoryComponent::new(config.clone(), &service);
        let cpu = CPUComponent::new(config.clone(), &service);
        let temp = TempComponent::new(config.clone(), &service);
        
        let focus = MainFocus::Process;
        let focus_rects = HashMap::new();

        Self {
            focus,
            focus_rects,
            expand: false,
            service,
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
        self.service.refresh_all();

        self.process.refresh(&self.service);
        self.memory.update(&self.service);
        self.cpu.update(&self.service);
        self.temp.refresh(&self.service);

        Ok(EventState::Consumed)
    }

    fn toggle_expand(&mut self) {
        self.expand = !self.expand
    }

    pub fn key_event(&mut self, key: Key) -> Result<EventState> {
        if self.component_event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        else if self.move_focus_key(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        else if key == self.config.key_config.expand {
            self.toggle_expand();

            return Ok(EventState::Consumed);
        }

        Ok(EventState::NotConsumed)
    }

    pub fn mouse_event(&mut self, mouse: Mouse) -> Result<EventState> {
        //TODO:
        //      1. change component_event -> component_key_event
        //      2. create component_mouse_event
        //      3. figure out how to select Cells? e.g., selecting a process in the process list
        match self.focus {
            MainFocus::Process => {
                if self.process.mouse_event(mouse)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            _ => {}
        }

        if self.move_focus_mouse(mouse)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        if matches!(mouse.kind, MouseKind::MiddleClick) {
            self.toggle_expand();
            return Ok(EventState::Consumed)
        }

        Ok(EventState::NotConsumed)
    }

    fn move_focus_mouse(&mut self, mouse: Mouse) -> Result<EventState> {
        if matches!(mouse.kind, MouseKind::LeftClick) {
            let col = mouse.column;
            let row = mouse.row;

            for (focus, rect) in &self.focus_rects {
                if rect.contains(col, row) {
                    self.focus = *focus;
                    return Ok(EventState::Consumed)
                }
            }
        }
        
        return Ok(EventState::NotConsumed)
    }

    fn component_event(&mut self, key: Key) -> Result<EventState> {
        if self.error.key_event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        if self.help.key_event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        match self.focus {
            MainFocus::CPU => {
                if self.cpu.key_event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Memory => {
                if self.memory.key_event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Temp => {
                if self.temp.key_event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Process => {
                if self.process.key_event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
                // terminate case
                if key == self.config.key_config.terminate {
                    //self.process.terminate_process(&self.system_wrapper);

                    return Ok(EventState::Consumed)
                }
            }
        }

        Ok(EventState::NotConsumed)
    }

    fn move_focus_key(&mut self, key: Key) -> Result<EventState> {
        if key == self.config.key_config.tab {
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
                self.focus_rects.insert(MainFocus::Process, chunks[0]);
            }

            if matches!(self.focus, MainFocus::CPU) {
                self.cpu.draw(
                    f,
                    chunks[0],
                    true,
                )?;
                self.focus_rects.insert(MainFocus::CPU, chunks[0]);
            }

            if matches!(self.focus, MainFocus::Memory) {
                self.memory.draw(
                    f,
                    chunks[0],
                    true,
                )?;
                self.focus_rects.insert(MainFocus::Memory, chunks[0]);
            }

            if matches!(self.focus, MainFocus::Temp) {
                self.temp.draw(
                    f,
                    chunks[0],
                    true,
                )?;
                self.focus_rects.insert(MainFocus::Temp, chunks[0]);
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
            self.focus_rects.insert(MainFocus::Process, vertical_chunks[2]);

            self.cpu.draw(
                f,
                vertical_chunks[0],
                matches!(self.focus, MainFocus::CPU)
            )?;
            self.focus_rects.insert(MainFocus::CPU, vertical_chunks[0]);

            self.memory.draw(
                f,
                horizontal_chunks[1][0],
                //vertical_chunks[1],
                matches!(self.focus, MainFocus::Memory)
            )?;
            self.focus_rects.insert(MainFocus::Memory, horizontal_chunks[1][0]);

            self.temp.draw(
                f,
                horizontal_chunks[1][1],
                matches!(self.focus, MainFocus::Temp)
            )?;
            self.focus_rects.insert(MainFocus::Temp, horizontal_chunks[1][1]);
        }

        //self.help.draw(f, Rect::default(), false)?; //TODO: re-implement

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

trait Contains {
    fn contains(&self, col: u16, row: u16) -> bool;
}
impl Contains for Rect {
    fn contains(&self, col: u16, row: u16) -> bool {
        col >= self.x
            && col < self.x + self.width
            && row >= self.y
            && row < self.y + self.height
    }
}