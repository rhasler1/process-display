use anyhow::{Ok, Result};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    prelude::*,
};
use crate::config::{Config, KeyConfig};
use process_list::{ListSortOrder, ProcessList, ProcessListItem, ProcessListItems};
use super::{
    common_nav, common_sort, Component, DrawableComponent, EventState,
    utils::vertical_scroll::VerticalScroll,
    filter::FilterComponent,
};

#[derive(PartialEq, Clone)]
pub enum Focus {
    Filter,
    List,
}

pub struct ProcessComponent {
    focus: Focus,
    list: ProcessList,
    filter: FilterComponent,
    filtered_list: Option<ProcessList>,
    scroll: VerticalScroll,
    pub config: Config,
}

impl ProcessComponent {
    pub fn new(config: Config, processes: Vec<ProcessListItem>) -> Self {
        Self {
            focus: Focus::List,
            list: ProcessList::new(processes),
            filter: FilterComponent::new(config.clone()),
            filtered_list: None,
            scroll: VerticalScroll::new(),
            config,
        }
    }

    pub fn update(&mut self, new_processes: Vec<ProcessListItem>) {
        assert!(!new_processes.is_empty());

        let dup = new_processes.clone();
    
        self.list.update(new_processes);   

        if let Some(filtered_list) = self.filtered_list.as_mut() {
            let processes = ProcessListItems::new(dup);
            let filtered_processes = processes.filter(self.filter.input_str());

            filtered_list.update(filtered_processes.list_items);
        }
    }

    // gets the selected process pid, returns Some(pid) or None
    pub fn selected_pid(&self) -> Option<u32> {
        if matches!(self.focus, Focus::List) {
            if let Some(filtered_list) = self.filtered_list.as_ref() {
                return filtered_list.selected_pid()
            }
            else {
                return self.list.selected_pid()
            }
        }
        None
    }
}

impl Component for ProcessComponent {
    fn event(&mut self, key: KeyEvent) -> Result<EventState> {
        if key.code == self.config.key_config.filter && self.focus == Focus::List {
            self.focus = Focus::Filter;
            
            return Ok(EventState::Consumed)
        }

        if matches!(self.focus, Focus::Filter) {
            if self.filter.event(key)?.is_consumed() {
                self.filtered_list = if self.filter.input_str().is_empty() {
                    None
                }
                else {
                    Some(self.list.filter(self.filter.input_str()))
                };

                return Ok(EventState::Consumed)
            }
            
            if key.code == self.config.key_config.enter {
                self.focus = Focus::List;

                return Ok(EventState::Consumed)
            }
        }

        if matches!(self.focus, Focus::List) {
            if list_nav(
                if let Some(list) = self.filtered_list.as_mut() {
                    list
                }
                else {
                    &mut self.list
                },
                key,
                &self.config.key_config
            ) {
                return Ok(EventState::Consumed);
            }

            if key.code == self.config.key_config.follow_selection {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.toggle_follow_selection();
                }
                else {
                    self.list.toggle_follow_selection();
                }

                return Ok(EventState::Consumed)
            }

            else if list_sort(
                if let Some(list) = self.filtered_list.as_mut() {
                    list
                }
                else {
                    &mut self.list
                },
                key,
                &self.config.key_config
            )? {
                return Ok(EventState::Consumed);
            }
        }
        
        Ok(EventState::NotConsumed)
    }
}

fn list_nav(list: &mut ProcessList, key: KeyEvent, key_config: &KeyConfig) -> bool {
    if let Some(move_dir) = common_nav(key, key_config) {
        list.move_selection(move_dir);

        true
    }
    else {
        false
    }
}

fn list_sort(list: &mut ProcessList, key: KeyEvent, key_config: &KeyConfig) -> Result<bool> {
    if let Some(sort) = common_sort(key, key_config) {
        list.sort(&sort);

        Ok(true)
    }
    else {
        Ok(false)
    }
}

impl DrawableComponent for ProcessComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()> {
        let horizontal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),        //list
                Constraint::Length(3),      //filter
            ]).split(area);

        let visible_list_height = horizontal_chunks[0].height.saturating_sub(3) as usize;

        let list = if let Some(filtered_list) = self.filtered_list.as_ref() {
            filtered_list
        }
        else {
            &self.list
        };

        list.selection().map_or_else(
            { ||
                self.scroll.reset()
            }, |selection| {
                self.scroll.update(
                    selection,
                    list.len(),
                    visible_list_height,
                );
            },
        );

        let visible_items = list
            .iterate(
                self.scroll.get_top(),
                visible_list_height,
            );

        draw_process_list(
            f, 
            horizontal_chunks[0], 
            visible_items, 
            list.is_follow_selection(),
            if focused {
                matches!(self.focus, Focus::List)
            } 
            else {
                false
            },
            self.config.theme_config.clone(),
            list.get_sort_order(),
        );

        self.scroll.draw(
            f, 
            horizontal_chunks[0],
            if focused {
                matches!(self.focus, Focus::List)
            } 
            else {
                false
            },
        )?;
                
        self.filter.draw(
            f, 
            horizontal_chunks[1],
            if focused {
                matches!(self.focus, Focus::Filter)
            } 
            else {
                false
            },
        )?;

        Ok(())
    }
}

use ratatui::widgets::{block::*, *};
use process_list::ListIterator;
use crate::config::ThemeConfig;

fn draw_process_list(
    f: &mut Frame,
    area: Rect,
    visible_items: ListIterator<'_>,
    follow_selection: bool,
    focus: bool,
    theme_config: ThemeConfig,
    sort_order: &ListSortOrder,
) {
    let follow_flag = follow_selection;

    // setting header
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

#[cfg(test)]
mod test {
    // TODO: write tests
}
