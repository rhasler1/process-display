use anyhow::Ok;
use ratatui::{style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListItem, ListState}};

use crate::config::{self, Config};

use super::{Component, DrawableComponent};

#[derive(Default)]
pub struct SystemComponentInner {
    kernel_version: String,
    host_name: String,
    cpu_architecture: String,
    physical_core_count: String,
    os_version: String,
    system_uptime: String,
}

impl SystemComponentInner {
    pub const FIELD_NAMES: [&str; 6] = [
        "Kernel Version",
        "Host Name",
        "CPU Architecture",
        "Physical Cores",
        "OS Version",
        "System Uptime",
    ];

    pub fn as_vec(&self) -> Vec<&str> { // read only
        let vec: Vec<&str> = vec![
            &self.kernel_version,
            &self.host_name,
            &self.cpu_architecture,
            &self.physical_core_count,
            &self.os_version,
            &self.system_uptime,
        ];

        vec
    }

    pub fn update(
        &mut self,
        system_uptime: String,
    ) {
        self.system_uptime = system_uptime;
    }
}

#[derive(Default)]
pub struct SystemComponent {
    system_inner: SystemComponentInner,
    config: Config,
    selection_inner: Option<usize>,
}

impl SystemComponent {
    pub fn init(
        &mut self,
        vec: Vec<String>,
    ) {
        let mut iter = vec.into_iter();

        self.system_inner.kernel_version = iter.next().unwrap();
        self.system_inner.host_name = iter.next().unwrap();
        self.system_inner.cpu_architecture = iter.next().unwrap();
        self.system_inner.physical_core_count = iter.next().unwrap();
        self.system_inner.os_version = iter.next().unwrap();
        self.system_inner.system_uptime = iter.next().unwrap();
        self.selection_inner = Some(0);
    }

    pub fn update(
        &mut self,
        system_uptime: String,
    ) {
        self.system_inner.system_uptime = system_uptime;
    }
}

impl Component for SystemComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<super::EventState> {
        if key.code == self.config.key_config.move_down {
            if let Some(s) = self.selection_inner {
                if s < SystemComponentInner::FIELD_NAMES.len() - 1 {
                    self.selection_inner = Some(s.saturating_add(1));
                }
                return Ok(super::EventState::Consumed)
            }
        }
        if key.code == self.config.key_config.move_up {
            if let Some(s) = self.selection_inner {
                self.selection_inner = Some(s.saturating_sub(1));
                return Ok(super::EventState::Consumed)
            }
        }
        Ok(super::EventState::NotConsumed)
    }
}

impl DrawableComponent for SystemComponent {
    fn draw(&mut self,
        f: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        focused: bool
    ) -> anyhow::Result<()> {
        let items: Vec<ListItem> = SystemComponentInner::FIELD_NAMES
            .iter()
            .zip(self.system_inner.as_vec().iter())
            .map(|(label, item)| ListItem::new(format!("{label}: {item}")).style(Color::White))
            .collect();

        let mut list_state = ListState::default();
        list_state.select(self.selection_inner);

        let list = List::new(items)
            .scroll_padding(area.height as usize / 2)
            .block(
                {
                    if !focused {
                        Block::default()
                            .title(" System Info ")
                            .borders(Borders::ALL)
                            .style(Color::DarkGray)
                    }
                    else {
                        Block::default()
                            .title(" System Info ")
                            .borders(Borders::ALL)
                            .style(Color::LightGreen)
                    }
                }
            )
            .highlight_style(
                Style::default().bg(Color::LightBlue).add_modifier(Modifier::BOLD)
            );

        f.render_stateful_widget(list, area, &mut list_state);

        Ok(())
    }
}
