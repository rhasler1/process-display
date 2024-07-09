use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
    text::Span,
};

use super::{StatefulDrawableComponent, Component, EventState};
use crate::config::KeyConfig;

#[derive(Clone)]
pub enum Tab {
    CPU,
    Memory,
}

pub struct TabComponent {
    pub selected_tab: Tab,
    key_config: KeyConfig
}

impl TabComponent {
    // default constructor
    pub fn new() -> Self {
        Self {
            selected_tab: Tab::CPU,
            key_config: KeyConfig::default(),
        }
    }

    // set internal TabComponent State to default
    pub fn reset(&mut self) {
        self.selected_tab = Tab::CPU;
        self.key_config = KeyConfig::default();
    }

    // String representation of Tab variants used in self.draw()
    fn names(&self) -> Vec<String> {
        vec![
            String::from("CPU"),
            String::from("Memory"),
        ]
    }

    // rotate between Tab variants
    fn update_selected_tab(&mut self) {
        match self.selected_tab {
            Tab::CPU => {
                self.selected_tab = Tab::Memory;
            }
            Tab::Memory => {
                self.selected_tab = Tab::CPU;
            }
        }
    }
}

impl Component for TabComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> std::io::Result<EventState> {
        if key.code == self.key_config.tab_right {
            // update selected tab -> only one key_config used here, self.update_selected_tab()
            // rotates between possible Tab variants
            self.update_selected_tab();
            return Ok(EventState::Consumed);
        }
        return Ok(EventState::NotConsumed);
    }
}

impl StatefulDrawableComponent for TabComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> std::io::Result<()> {
        let title: &str = "List Type";

        let names: Vec<String> = self.names();
        let titles: Vec<Line> = names
            .iter()
            .map(
                |name|
                Line::from(
                    Span::raw(
                        name.clone()
                    )
                )
            )
            .collect();

        let selected_tab = self.selected_tab.clone() as usize;

        let selected_tab_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);

        let other_tab_style = Style::default().fg(Color::DarkGray);

        let tabs: Tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title(title))
            .select(selected_tab)
            .style(other_tab_style)
            .highlight_style(selected_tab_style);

        f.render_widget(tabs, area);

        return Ok(())
    }
}