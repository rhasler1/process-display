use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
    text::Span,
};
use super::{DrawableComponent, Component, EventState};
use crate::config::KeyConfig;

#[derive(Clone, PartialEq)]
enum MoveTabDirection {
    Left,
    Right,
}

#[derive(Clone)]
pub enum Tab {
    Process,
    Performance,
    //Users,
}

pub struct TabComponent {
    pub selected_tab: Tab,
    key_config: KeyConfig,
}

impl TabComponent {
    // default constructor
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            selected_tab: Tab::Process,
            key_config: key_config,
        }
    }

    // set internal TabComponent State to default
    pub fn reset(&mut self) {
        self.selected_tab = Tab::Process;
        self.key_config = KeyConfig::default();
    }

    // String representation of Tab variants used in self.draw()
    fn names(&self) -> Vec<String> {
        vec![
            String::from("Process"),
            String::from("Performance"),
            //String::from("Users"),
        ]
    }

    fn update_selected_tab(&mut self, direction: MoveTabDirection) {
        match self.selected_tab {
            Tab::Process => {
                if direction == MoveTabDirection::Right {
                    self.selected_tab = Tab::Performance;
                }
                else {
                    self.selected_tab = Tab::Performance;
                }
            }
            Tab::Performance => {
                if direction == MoveTabDirection::Right {
                    self.selected_tab = Tab::Process;
                }
                else {
                    self.selected_tab = Tab::Process;
                }
            }
            //Tab::Users => {
            //    if direction == MoveTabDirection::Right {
            //        self.selected_tab = Tab::Process;
            //    }
            //    else {
            //        self.selected_tab = Tab::Performance;
            //    }
            //}
        }
    }
}

impl Component for TabComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> std::io::Result<EventState> {
        if key.code == self.key_config.tab_right {
            self.update_selected_tab(MoveTabDirection::Right);
            return Ok(EventState::Consumed);
        }
        else if key.code == self.key_config.tab_left {
            self.update_selected_tab(MoveTabDirection::Left);
            return Ok(EventState::Consumed);
        }
        return Ok(EventState::NotConsumed);
    }
}

impl DrawableComponent for TabComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> std::io::Result<()> {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // filter and tab chunk
                Constraint::Min(1) // list chunk
            ].as_ref())
            .split(area);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // space for tab
                Constraint::Percentage(50), // space for filter
            ].as_ref())
            .split(vertical_chunks[0]);

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

        f.render_widget(tabs, horizontal_chunks[0]);

        return Ok(())
    }
}