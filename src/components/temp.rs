use anyhow::{Ok, Result};
use ratatui::widgets::{Block, Borders, Table};
use ratatui::prelude::*;
use ratatui::Frame;
use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use crossterm::event::KeyEvent;
use crate::components::sysinfo_wrapper::SysInfoWrapper;
use crate::components::utils::vertical_scroll::VerticalScroll;
use crate::models::items::temp_item::TempItem;
use crate::{components::EventState, config::Config};
use super::{Component, DrawableComponent};

pub struct TempComponent {
    config: Config,
    temps: Vec<TempItem>,
    ui_selection: usize,
    vertical_scroll: VerticalScroll,
}

impl TempComponent {
    pub fn new(config: Config, sysinfo: &SysInfoWrapper) -> Self {
        let mut temps = Vec::new();
        sysinfo.get_temps(&mut temps);

        Self {
            config,
            temps,
            ui_selection: 0,
            vertical_scroll: VerticalScroll::new(),
        }
    }

    pub fn update(&mut self, sysinfo: &SysInfoWrapper) {
        sysinfo.get_temps(&mut self.temps);
    }
}

impl Component for TempComponent {
    fn event(&mut self, key: KeyEvent) -> Result<EventState> {
        let temps_max_idx = self.temps.len().saturating_sub(1);

        if key.code == self.config.key_config.move_down {
            if self.ui_selection < temps_max_idx {
                self.ui_selection = self.ui_selection.saturating_add(1);
            }
            return Ok(super::EventState::Consumed);
        }
        if key.code == self.config.key_config.move_up {
            self.ui_selection = self.ui_selection.saturating_sub(1);
            return Ok(super::EventState::Consumed);
        }
        
        Ok(super::EventState::NotConsumed)
    }
}

//TODO: take a closer look at:
//  1. VerticalScroll
//  2. ListIter & ListItemsIter
impl DrawableComponent for TempComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()> {
        let visible_height = area.height.saturating_sub(3) as usize;
        
        self.vertical_scroll.update(
            self.ui_selection,
            self.temps.len(),
            visible_height,
        );

        let visible_items = self.temps.iter().skip(self.vertical_scroll.get_top()).take(visible_height);

        let header = ["Sensor", "Temp", "Max Temp", "Critical Temp"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(
                if focused {
                    self.config.theme_config.style_border_focused
                }
                else {
                    self.config.theme_config.style_border_not_focused
                }
            )
            .height(1);

        let rows = visible_items
            .map(|item| {
                let style: Style =
                    if focused && item.label().eq(self.temps[self.ui_selection].label()) {
                        self.config.theme_config.style_item_selected       
                    }
                    else if focused {
                        self.config.theme_config.style_item_focused
                    }
                    else if !focused && item.label().eq(self.temps[self.ui_selection].label()) {
                        self.config.theme_config.style_item_selected_not_focused
                    }
                    else {
                        self.config.theme_config.style_item_not_focused
                    };
                
                let cells: Vec<Cell> = vec![
                    Cell::from(item.label().to_string()),
                    Cell::from(item.temp().to_string()),
                    Cell::from(item.max_temp().to_string()),
                    Cell::from(item.critical_temp().to_string()),
                ];

                Row::new(cells).style(style)
            })
            .collect::<Vec<_>>();

        let widths = vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ];

        let block_title: &str = " Sensor Temperatures ";
        let block_style = if focused {
            self.config.theme_config.style_border_focused
        }
        else {
            self.config.theme_config.style_border_not_focused
        };

        let table = Table::new(rows, widths)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(block_title))
            .style(block_style);

        f.render_widget(table, area);
        
        Ok(())
    }
}