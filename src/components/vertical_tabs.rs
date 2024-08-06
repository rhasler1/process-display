use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
    text::Span,
};
use sysinfo::Cpu;
use super::{DrawableComponent, Component, EventState};
use crate::config::KeyConfig;

#[derive(Clone, PartialEq)]
pub enum MoveTabDirection {
    Up,
    Down,
}

#[derive(Clone)]
pub enum VerticalTab {
    Cpu,
    Memory,
    Network,
}

impl Default for VerticalTab {
    fn default() -> Self {
        VerticalTab::Cpu
    }
}

#[derive(Default)]
pub struct VerticalTabComponent {
    pub selected_vert_tab: VerticalTab,
    key_config: KeyConfig,
}

impl VerticalTabComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            selected_vert_tab: VerticalTab::Cpu,
            key_config: key_config,
        }
    }

    pub fn reset(&mut self) {
        self.selected_vert_tab = VerticalTab::Cpu;
        self.key_config = KeyConfig::default();
    }

    fn names(&self) -> Vec<String> {
        vec![
            String::from("CPU"),
            String::from("Memory"),
            String::from("Network"),
        ]
    }

    fn update_selected_tab(&mut self, direction: MoveTabDirection) {
        match self.selected_vert_tab {
            VerticalTab::Cpu => {
                if direction == MoveTabDirection::Up {
                    self.selected_vert_tab = VerticalTab::Network;
                }
                else {
                    self.selected_vert_tab = VerticalTab::Memory;
                }
            }
            VerticalTab::Memory => {
                if direction == MoveTabDirection::Up {
                    self.selected_vert_tab = VerticalTab::Cpu;
                }
                else {
                    self.selected_vert_tab = VerticalTab::Network;
                }
            }
            VerticalTab::Network => {
                if direction == MoveTabDirection::Up {
                    self.selected_vert_tab = VerticalTab::Memory;
                }
                else {
                    self.selected_vert_tab = VerticalTab::Cpu;
                }
            }
        }
    }   
}

impl Component for VerticalTabComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> std::io::Result<EventState> {
        if key.code == self.key_config.move_up {
            self.update_selected_tab(MoveTabDirection::Up);
            return Ok(EventState::Consumed);
        }
        else if key.code == self.key_config.move_down {
            self.update_selected_tab(MoveTabDirection::Down);
            return Ok(EventState::Consumed);
        }
        return Ok(EventState::NotConsumed)
    }
}

impl DrawableComponent for VerticalTabComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> std::io::Result<()> {
        let selected_tab = self.selected_vert_tab.clone() as usize;
        let selected_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
        let default_style = Style::default().fg(Color::DarkGray);

        let titles: Vec<Line> = self.names()
            .iter()
            .enumerate()
            .map(|(idx, name)|
                Line::from(
                    if idx == selected_tab {
                        Span::styled(name.clone(), selected_style)
                    }
                    else {
                        Span::styled(name.clone(), default_style)
                    }
                )
            )
            .collect();

        let widget = Paragraph::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Performance Focus"))
            .style(Style::default().fg(Color::DarkGray));

        f.render_widget(widget, area);

        Ok(())
    }
}