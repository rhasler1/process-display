use std::collections::HashMap;
use anyhow::{Ok, Result};
use ratatui::prelude::*;
use crate::components::help::HelpComponent;
use crate::input::{Key, Mouse, MouseKind};
use crate::components::{command, Refreshable};
use crate::config::{Config, KeyConfig, MouseConfig};
use crate::components::{
    cpu::CPUComponent,
    memory::MemoryComponent,
    network::NetworkComponent,
    process::ProcessComponent,
    error::ErrorComponent,
    EventState,
    Component,
    DrawableComponent,
    //help::HelpComponent,
};
use crate::components::command::CommandInfo;
use crate::services::sysinfo_service::SysInfoService;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum MainFocus {
    CPU,
    Process,
    Memory,
    Network,
    Help,
}

pub struct App {
    focus: MainFocus,
    focus_rects: HashMap<MainFocus, Rect>,
    expand: bool,
    service: SysInfoService,
    process: ProcessComponent,
    cpu: CPUComponent,
    memory: MemoryComponent,
    network: NetworkComponent,
    //temp: TempComponent,
    help: HelpComponent,
    freeze_flag: bool,
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
        let network = NetworkComponent::new(config.clone(), &service);
        //let temp = TempComponent::new(config.clone(), &service);

        let help_config = config.clone();
        let mut help = HelpComponent::new(help_config.clone());
        help.set_commands(commands(&help_config.key_config, &help_config.mouse_config));
        
        let focus = MainFocus::Process;
        let focus_rects = HashMap::new();
        let freeze_flag = false;

        Self {
            focus,
            focus_rects,
            expand: false,
            service,
            process,
            cpu,
            memory,
            network,
            //temp,
            help,
            freeze_flag,
            error: ErrorComponent::new(config.clone()),
            config: config.clone(),
        }
    }

    pub fn toggle_freeze(&mut self) {
        self.freeze_flag = !self.freeze_flag;
    }

    pub fn refresh_event(&mut self) -> Result<EventState> {
        if self.freeze_flag {
            return Ok(EventState::Consumed)
        }

        self.service.refresh_all();
        self.process.refresh(&self.service);
        self.memory.refresh(&self.service);
        self.cpu.update(&self.service);
        self.network.refresh(&self.service);
        //TODO: self.clock.refresh();

        Ok(EventState::Consumed)
    }

    fn toggle_expand(&mut self) {
        self.expand = !self.expand
    }

    pub fn key_event(&mut self, key: Key) -> Result<EventState> {
        if self.help.is_visible() {
            let _ = self.help.key_event(key)?.is_consumed();
            return Ok(EventState::Consumed)
        }

        if self.key_component_event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        if self.move_focus_key(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        if key == self.config.key_config.expand {
            self.toggle_expand();
            return Ok(EventState::Consumed);
        }
        if key == self.config.key_config.freeze {
            self.toggle_freeze();
            return Ok(EventState::Consumed);
        }

        Ok(EventState::NotConsumed)
    }

    fn key_component_event(&mut self, key: Key) -> Result<EventState> {
        if self.error.key_event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }
        //if self.help.key_event(key)?.is_consumed() {
        //    return Ok(EventState::Consumed)
        //}
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
            MainFocus::Network => {
                if self.network.key_event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Process => {
                if self.process.key_event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
                // terminate case
                if key == self.config.key_config.terminate {
                    
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Help => {
                if self.help.key_event(key)?.is_consumed() {
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
                    self.focus = MainFocus::Network
                }
                MainFocus::Network => {
                    self.focus = MainFocus::Process
                }
                MainFocus::Process => {
                    self.focus = MainFocus::CPU
                }
                _ => {}
            }
            return Ok(EventState::Consumed)
        }

        Ok(EventState::NotConsumed)
    }

    pub fn mouse_event(&mut self, mouse: Mouse) -> Result<EventState> {
        // mouse event can result in multiple state changes
        // for example: The app's main focus is on CPU. A left-click
        // mouse event occurs on the proces list. This one event will
        // first change the app's main focus to the ProcessComponent,
        // then the ProcessComponent will handle the mouse click to potentially
        // change it's own focus state (List, Filter) or selection state.
        if self.help.is_visible() {
            let _ = self.help.mouse_event(mouse)?.is_consumed();
            return Ok(EventState::Consumed)
        }

        let move_focus_res = self.move_focus_mouse(mouse)?.is_consumed();

        match self.focus {
            MainFocus::Process => {
                if self.process.mouse_event(mouse)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::CPU => {
                if self.cpu.mouse_event(mouse)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Memory => {
                if self.memory.mouse_event(mouse)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Network => {
                if self.network.mouse_event(mouse)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            MainFocus::Help => {
                if self.help.mouse_event(mouse)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
        }

        if move_focus_res {
            return Ok(EventState::Consumed)
        }

        Ok(EventState::NotConsumed)
    }

    fn move_focus_mouse(&mut self, mouse: Mouse) -> Result<EventState> {
        if matches!(mouse.kind, MouseKind::LeftClick) {
            let col = mouse.column;
            let row = mouse.row;

            //special case for help: (help rectangle intersects process rectangle)
            //this is a small intersection where help should take priority
            if let Some(rect) = self.focus_rects.get(&MainFocus::Help) {
                if rect.contains(col, row) {
                    return Ok(EventState::Consumed)
                }
            }

            for (focus, rect) in &self.focus_rects {
                if rect.contains(col, row) {
                    self.focus = *focus;
                    return Ok(EventState::Consumed)
                }
            }
        }
        
        return Ok(EventState::NotConsumed)
    }

    pub fn draw(&mut self, f: &mut Frame) -> Result<()> {
        self.focus_rects.clear();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
            ])
            .split(f.size());

        self.error.draw(f, chunks[0], false)?;
        
        if self.help.is_visible() {
            self.help.draw(f, chunks[0], false)?;
            return Ok(())
        }

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

            if matches!(self.focus, MainFocus::Network) {
                self.network.draw(
                    f,
                    chunks[0],
                    true,
                )?;
                self.focus_rects.insert(MainFocus::Network, chunks[0]);
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

            let process_horizontal_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),        //table(process list)
                    Constraint::Length(3),      //filter
                ]).split(vertical_chunks[2]);

            // splitting filter for help
            let bottom_horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(95), // filter
                    Constraint::Percentage(5),  // help
                ])
                .split(process_horizontal_chunks[1]);

            self.process.draw(
                f,
                vertical_chunks[2],
                matches!(self.focus, MainFocus::Process)
            )?;
            self.focus_rects.insert(MainFocus::Process, vertical_chunks[2]);

            self.help.draw(
                f,
                bottom_horizontal_chunks[1],
                self.help.is_visible(),
            )?;
            self.focus_rects.insert(MainFocus::Help, bottom_horizontal_chunks[1]);

            self.cpu.draw(
                f,
                vertical_chunks[0],
                matches!(self.focus, MainFocus::CPU)
            )?;
            self.focus_rects.insert(MainFocus::CPU, vertical_chunks[0]);

            self.memory.draw(
                f,
                horizontal_chunks[1][0],
                matches!(self.focus, MainFocus::Memory)
            )?;
            self.focus_rects.insert(MainFocus::Memory, horizontal_chunks[1][0]);

            self.network.draw(
                f,
                horizontal_chunks[1][1],
                matches!(self.focus, MainFocus::Network)
            )?;
            self.focus_rects.insert(MainFocus::Network, horizontal_chunks[1][1]);
        }

        return Ok(())
    }
}

fn commands(key_config: &KeyConfig, mouse_config: &MouseConfig) -> Vec<CommandInfo> {
    let res = vec![
        CommandInfo::new(command::help(key_config)),
        CommandInfo::new(command::exit_popup(key_config)),
        //CommandInfo::new(command::change_tab(&self.config.key_config)),
        CommandInfo::new(command::move_selection(key_config)),
        CommandInfo::new(command::selection_to_top_bottom(key_config)),
        CommandInfo::new(command::sort_list_by_name(key_config, mouse_config)),
        CommandInfo::new(command::sort_list_by_pid(key_config, mouse_config)),
        CommandInfo::new(command::sort_list_by_cpu_usage(key_config, mouse_config)),
        CommandInfo::new(command::sort_list_by_memory_usage(key_config, mouse_config)),
        CommandInfo::new(command::filter_submit(key_config)),
        CommandInfo::new(command::terminate_process(key_config)),
    ];

    res
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