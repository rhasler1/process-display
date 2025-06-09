// make similar to CPU

/*

pub struct CPUComponent {
    cpus: BTreeMap<usize, BoundedQueue<CpuItem>>,
    ui_selection: usize,
    config: Config,
}

*/
use std::collections::BTreeMap;
use bounded_queue::BoundedQueue;
use crate::{components::EventState, config::Config};
use bounded_queue::TempItem;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, List, ListItem, ListState, Table};
use anyhow::{Ok, Result};
use super::{Component, DrawableComponent};
use ratatui::{
    style::{Style, Stylize},
};

pub struct TempComponent {
    config: Config,
    temps: BTreeMap<String, BoundedQueue<TempItem>>,
    ui_selection: usize,
}

impl TempComponent {
    pub fn new(config: Config) -> Self {
        let temps: BTreeMap<String, BoundedQueue<TempItem>> = BTreeMap::new();

        Self {
            config,
            temps,
            ui_selection: 0,
        }
    }

    pub fn update(&mut self, temp_items: Vec<TempItem>) {
        for temp_item in temp_items {
            let key = temp_item.label().to_string();

            let queue = self.temps.entry(key).or_insert_with(|| {
                BoundedQueue::new(self.config.events_per_min() as usize)
            });

            queue.add_item(temp_item);
        }
    }
}

impl Component for TempComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> Result<EventState> {
        let temps_max_idx = self.temps.len() - 1;

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


// I can't get critical temp values on Mac and potentially Windows.
// Instead of doing gauges for temp, I'm just going to report as a table.
// Also, sorting by String, messes up the order e.g., item0 -> item1 -> item10 -> item2
// TODO: Do not use boundedqueue, just create a Vector of TempItems.
/*
  let header = ["",
        if matches!(sort_order, ListSortOrder::PidInc) { "PID ▲" }
        else if matches!(sort_order, ListSortOrder::PidDec) { "PID ▼" }
        else { "PID" },

        if matches!(sort_order, ListSortOrder::NameInc) { "Name ▲" } 
        else if matches!(sort_order, ListSortOrder::NameDec) { "Name ▼" }
        else { "Name" },

        if matches!(sort_order, ListSortOrder::CpuUsageInc) { "CPU (%) ▲" }
        else if matches!(sort_order, ListSortOrder::CpuUsageDec) { "CPU (%) ▼" }
        else { "CPU (%)" },

        if matches!(sort_order, ListSortOrder::MemoryUsageInc) { "Memory (MB) ▲" }
        else if matches!(sort_order, ListSortOrder::MemoryUsageDec) { "Memory (MB) ▼" }
        else { "Memory (MB)" },

        "Run (hh:mm:ss)",
        "Status",
        "Path"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(
            if focus {
                theme_config.style_border_focused
            }
            else {
                theme_config.style_border_not_focused
            }
        )
        .height(1);

    // setting rows
    let rows = visible_items
        .map(|(item, selected)| {
            let style =
                if focus && selected && follow_flag {
                    theme_config.style_item_selected_followed
                }
                else if focus && selected && !follow_flag {
                    theme_config.style_item_selected
                }
                else if focus {
                    theme_config.style_item_focused
                }
                else if !focus && selected & follow_flag {
                    theme_config.style_item_selected_followed_not_focused
                }
                else if !focus && selected & !follow_flag {
                    theme_config.style_item_selected_not_focused
                }
                else {
                    theme_config.style_item_not_focused
                }
            ;

            let cells: Vec<Cell> = vec![
                if style == theme_config.style_item_selected ||
                    style == theme_config.style_item_selected_followed || 
                    style == theme_config.style_item_selected_followed_not_focused || 
                    style == theme_config.style_item_selected_not_focused {
                        Cell::from(String::from("->"))
                }
                else {
                    Cell::from(String::from(""))
                },
                Cell::from(item.pid().to_string()),
                Cell::from(item.name().to_string()),
                Cell::from(format!("{:.2}", item.cpu_usage())),
                Cell::from(format!("{:.2}", item.memory_usage()/1000000)),
                Cell::from(item.run_time_hh_mm_ss()),
                Cell::from(item.status()),
                Cell::from(item.path()),
            ];
            Row::new(cells).style(style)
        })
        .collect::<Vec<_>>();

    // setting the width constraints.
    let widths =
    vec![
        Constraint::Length(2),
        Constraint::Length(10), // pid
        Constraint::Length(50), // name
        Constraint::Length(15), // cpu usage
        Constraint::Length(15), // memory usage
        Constraint::Length(20), // run time
        Constraint::Length(15), // status
        Constraint::Min(0),     // path
    ];

    // setting block information
    let block_title: &str = " Process List ";
    let block_style =
        if focus {
            theme_config.style_border_focused
        }
        else {
            theme_config.style_border_not_focused
        };

    // setting the table
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(block_title))
        .style(block_style);

    // render
    f.render_widget(table, area);
}
*/
impl DrawableComponent for TempComponent {
    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect, focused: bool) -> Result<()> {
        // get the temp component selected
        let selection = self.ui_selection;

        //let header = ["Sensor", "Temp", "Max Temp", "Critical Temp"];

        //let rows = self.temps
        //    .iter()
        //    .map(|item| {})

        //let table = Table::new(rows, widths)
            //.header(header)
         //   .block(Block::default().borders(Borders::ALL).title(block_title))
          //  .style(block_style);

    
        // populate names for UIList to draw temp component list
        let mut names: Vec<ListItem> = self.temps
            .iter()
            .map(|(key, queue)| {
                let title = format!("{} {}", key.to_string(), queue.back().unwrap().critical_temp());
                ListItem::new(title)
            })
            .collect();
        
        let mut list_state = ListState::default();
        
        list_state.select(Some(self.ui_selection));

        let temp_list = List::new(names)
            .scroll_padding(area.height as usize / 2)
            .block(
                if !focused {
                    Block::default().borders(Borders::ALL).style(self.config.theme_config.style_border_not_focused)
                }
                else {
                    Block::default().borders(Borders::ALL).style(self.config.theme_config.style_border_focused)
                }
            )
            .highlight_style(
                if !focused {
                    self.config.theme_config.style_item_selected_not_focused
                }
                else {
                    self.config.theme_config.style_item_selected
                }
            );
        
        f.render_stateful_widget(temp_list, area, &mut list_state);
        
        Ok(())
    }
}