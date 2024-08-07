use std::io::{self};
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use crate::config::Config;
use crate::components::{
    tab::TabComponent,
    process::ProcessComponent,
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
    tab: TabComponent,
    help: HelpComponent,
    pub error: ErrorComponent,
    pub config: Config,
}

impl App {
    // New constructor.
    pub fn new(config: Config) -> Self {
        Self {
            system: SystemComponent::new(config.key_config.clone()),
            process: ProcessComponent::new(config.key_config.clone()),
            tab: TabComponent::new(config.key_config.clone()),
            help: HelpComponent::new(config.key_config.clone()),
            error: ErrorComponent::new(config.key_config.clone()),
            config: config.clone(),
        }
    }

    pub async fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        self.update_commands();

        if self.components_event(key).await?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        else if self.move_focus(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        Ok(EventState::NotConsumed)
    }

    // This function populates the HelpComponent with CommandInfo.
    fn update_commands(&mut self) {
        self.help.set_commands(self.commands());
    }

    // This function populates and returns a vector with CommandInfo.
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

    // Async function to process component's event.
    async fn components_event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if self.error.event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        if self.help.event(key)?.is_consumed() {
            return Ok(EventState::Consumed)
        }

        if self.tab.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        match self.tab.selected_tab {
            Tab::Process => {
                if self.process.event(key)?.is_consumed() {
                    return Ok(EventState::Consumed);
                }
                //else if key.code == self.config.key_config.terminate {
                //    if let Some(pid) = self.process.selected_pid() {
                //        self.system.terminate_process(pid)?;
                //        return Ok(EventState::Consumed)
                //    }
                //}
            }

            Tab::Performance => {}

            Tab::Users => {}
        }
        Ok(EventState::NotConsumed)
    }

    fn move_focus(&mut self, _key: KeyEvent) -> io::Result<EventState> {
        return Ok(EventState::NotConsumed);
    }

    // Async function to refresh the system structure and update dependent components.
    pub async fn refresh(&mut self) -> io::Result<()> {
        self.system.refresh_all().await?;
        let new_processes = self.system.get_process_list();
        self.process.update(new_processes.as_ref())?;
        Ok(())
    }

    // App draw.
    pub fn draw(&mut self, f: &mut Frame) -> io::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
            ])
            .split(f.size());

        self.error.draw(f, chunks[0], false)?;
        
        self.tab.draw(f, chunks[0], false)?;

        match self.tab.selected_tab {
            Tab::Process => {
                self.process.draw(f, chunks[0], false)?;
            }

            Tab::Performance => {}

            Tab::Users => {}
        }

        // Drawing the HelpComponent as a pop up. See /components/help.rs.
        self.help.draw(f, Rect::default(), false)?;

        return Ok(())
    }
}