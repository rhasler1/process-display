use anyhow::Result;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
    text::Span,
};
use super::{DrawableComponent, Component, EventState};
use crate::config::Config;

#[derive(Clone, PartialEq)]
enum MoveTabDirection {
    Left,
    Right,
}

#[derive(Clone)]
pub enum Tab {
    Process,
    CPU,
    Memory,
    Disk,
    //Users,
}

pub struct TabComponent {
    pub selected_tab: Tab,
    pub config: Config,
}

impl TabComponent {
    // default constructor
    pub fn new(config: Config) -> Self {
        Self {
            selected_tab: Tab::Process,
            config: config,
        }
    }

    // set internal TabComponent State to default
    pub fn reset(&mut self) {
        self.selected_tab = Tab::Process;
        self.config = Config::default();
    }

    // String representation of Tab variants used in self.draw()
    fn names(&self) -> Vec<String> {
        vec![
            String::from("Process"),
            String::from("CPU"),
            String::from("Memory"),
            String::from("Disk"),
            //String::from("Users"),
        ]
    }

    fn update_selected_tab(&mut self, direction: MoveTabDirection) {
        match self.selected_tab {
            Tab::Process => {
                if direction == MoveTabDirection::Right {
                    self.selected_tab = Tab::CPU;
                }
                else {
                    self.selected_tab = Tab::Disk;
                }
            }
            Tab::CPU => {
                if direction == MoveTabDirection::Right {
                    self.selected_tab = Tab::Memory;
                }
                else {
                    self.selected_tab = Tab::Process;
                }
            }
            Tab::Memory => {
                if direction == MoveTabDirection::Right {
                    self.selected_tab = Tab::Disk;
                }
                else {
                    self.selected_tab = Tab::CPU;
                }
            }
            Tab::Disk => {
                if direction == MoveTabDirection::Right {
                    self.selected_tab = Tab::Process;
                }
                else {
                    self.selected_tab = Tab::Memory;
                }
            }
        }
    }
}

impl Component for TabComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> Result<EventState> {
        if key.code == self.config.key_config.tab_right {
            self.update_selected_tab(MoveTabDirection::Right);
            return Ok(EventState::Consumed);
        }
        else if key.code == self.config.key_config.tab_left {
            self.update_selected_tab(MoveTabDirection::Left);
            return Ok(EventState::Consumed);
        }
        return Ok(EventState::NotConsumed);
    }
}

impl DrawableComponent for TabComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> Result<()> {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // filter and tab chunk
                Constraint::Min(1), // list chunk
                Constraint::Length(3), // filter chunk
            ].as_ref())
            .split(area);

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
            .block(Block::default().borders(Borders::ALL))
            .select(selected_tab)
            .style(other_tab_style)
            .highlight_style(selected_tab_style);

        f.render_widget(tabs, vertical_chunks[0]);

        return Ok(())
    }
}