use anyhow::{Ok, Result};
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use crate::config::Config;
use crate::components::{
    tab::TabComponent,
    process::ProcessComponent,
    performance::PerformanceComponent,
    system::SystemComponent,
    error::ErrorComponent,
    Component,
    EventState,
    DrawableComponent,
    tab::Tab,
    help::HelpComponent,
    command,
    command::CommandInfo,
};

pub struct App {
    system: SystemComponent,
    process: ProcessComponent,
    performance: PerformanceComponent,
    tab: TabComponent,
    help: HelpComponent,
    pub error: ErrorComponent,
    pub config: Config,
}

impl App {
    // constructor.
    pub fn new(config: Config) -> Self {
        Self {
            system: SystemComponent::new(config.clone()),
            process: ProcessComponent::new(config.clone()),
            performance: PerformanceComponent::new(config.clone(), 10),
            tab: TabComponent::new(config.clone()),
            help: HelpComponent::new(config.clone()),
            error: ErrorComponent::new(config.clone()),
            config: config.clone(),
        }
    }

    // call after constructor
    pub fn init(&mut self) -> Result<()> {
        self.system.refresh_all()?;
        self.update_process();
        self.update_performance()?;
        self.update_commands();

        Ok(())
    }

    // refresh system and dependencies
    pub fn refresh(&mut self) -> Result<()>{
        self.system.refresh_all()?;
        self.update_process();
        self.update_performance()?;

        Ok(())
    }

    // return result of process update
    fn update_process(&mut self) -> bool {
        let new_processes = self.system.get_processes();
        let res = self.process.update(&new_processes);

        res
    }

    // fix return type
    fn update_performance(&mut self) -> Result<()> {
        let new_cpu_info = self.system.get_cpu_info();
        let new_memory_info = self.system.get_memory_info();
        self.performance.update(&new_cpu_info, &new_memory_info)?;

        Ok(())
    }

    // top level key event processor
    pub fn event(&mut self, key: KeyEvent) -> Result<EventState> {
        if key.code == self.config.key_config.toggle_themes {
            self.update_component_themes();

            return Ok(EventState::Consumed)
        }
        else if self.components_event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        else if self.move_focus(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        Ok(EventState::NotConsumed)
    }

    // toggle color scheme
    fn update_component_themes(&mut self) {
        self.config.theme_config.toggle_themes();
        self.process.config.theme_config.toggle_themes();
        self.performance.config.theme_config.toggle_themes();
        self.help.config.theme_config.toggle_themes();
        self.system._config.theme_config.toggle_themes();
        self.tab.config.theme_config.toggle_themes();
    }
    
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

    // component key event processor
    fn components_event(&mut self, key: KeyEvent) -> Result<EventState> {
        if self.error.event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        if self.help.event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        match self.tab.selected_tab {
            Tab::Process => {
                // see if key event is processed by process component
                if self.process.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
                // terminate system process
                else if key.code == self.config.key_config.terminate {
                    if let Some(pid) = self.process.selected_pid() {
                        self.system.terminate_process(pid)?;

                        return Ok(EventState::Consumed)
                    }
                }
            }
            Tab::Performance => {
                if self.performance.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed)
                }
            }
            //Tab::Users => {}
        }

        if self.tab.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
        
        Ok(EventState::NotConsumed)
    }

    // not being used, implement if there is a need for focus control outside of tab
    fn move_focus(&mut self, _key: KeyEvent) -> Result<EventState> {
        return Ok(EventState::NotConsumed);
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
        
        // always draw tab--identifies tab state
        self.tab.draw(f, chunks[0], false)?;

        // only draw selected tab
        match self.tab.selected_tab {
            Tab::Process => {
                self.process.draw(f, chunks[0], false)?;
            }
            Tab::Performance => {
                self.performance.draw(f, chunks[0], false)?;
            }
            //Tab::Users => {}
        }

        // if help--help component state determines if anything is drawn
        self.help.draw(f, Rect::default(), false)?;

        return Ok(())
    }
}