use std::io;
use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use super::{filter::FilterComponent, process_list::MoveSelection, Component, EventState, StatefulDrawableComponent};
use super::process_list_items::ProcessListItems;
use super::process_list::ProcessList;
use super::process_list_items::ProcessListItem;
use crate::config::KeyConfig;
use super::common_nav;

// can either focus on filter or unfiltered list
#[derive(PartialEq)]
pub enum Focus {
    Filter,
    List,
}

pub struct CPUComponent {
    focus: Focus,
    list: ProcessList,
    filter: FilterComponent,
    filtered_list: Option<ProcessList>,
    //TODO: scroll: VerticalScroll
    key_config: KeyConfig,
}

impl CPUComponent {
    // pub method to construct CPUComponent
    //
    pub fn new() -> Self {
        Self {
            focus: Focus::List,
            list: ProcessList::default(),
            filter: FilterComponent::new(),
            filtered_list: None,
            key_config: KeyConfig::default(),
        }
    }

    // pub function to update the process list
    //
    pub async fn update(&mut self, processes: &Vec<ProcessListItem>) {
        self.list = ProcessList::new(processes);
        self.filtered_list = None;
        self.filter.reset();
    }

    //  pub fn list -- getter
    //
    pub fn list(&self) -> &ProcessList {
        self.filtered_list.as_ref().unwrap_or(&self.list)
    }

    //
    pub fn list_focused(&self) -> bool {
        matches!(self.focus, Focus::List)
    }


}

impl Component for CPUComponent {
    // handle key events for CPUComponent
    //
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        //  if they key event is filter and the CPUComponent Focus is on the List, then move the focus to Filter and return.
        //
        if key.code == self.key_config.filter && self.focus == Focus::List {
            self.focus = Focus::Filter;
            return Ok(EventState::Consumed)
        }

        // if the CPUComponent Focus is on the Filter, then attempt to set the filtered_list.
        // if the filter's input string is None, then set the filtered_list to None (no List to display),
        // else create the filtered_list calling list.filter(input_str)
        //
        if matches!(self.focus, Focus::Filter) {
            self.filtered_list = if self.filter.input_str().is_empty() {
                None
            }
            else {
                Some(self.list.filter(self.filter.input_str()))
            };
        }

        // if the key event is enter and the focus is Filter, then change the focus to List and return.
        //
        if key.code == self.key_config.enter && matches!(self.focus, Focus::Filter) {
            self.focus = Focus::List;
            return Ok(EventState::Consumed)
        }

        // if the focus is Filter
        // pass the key event to self.filter and attempt to consume.
        //
        if matches!(self.focus, Focus::Filter) {
            if self.filter.event(key)?.is_consumed() {
                return Ok(EventState::Consumed)
            }
        }

        //  if the filtered_list is Some pass it as argument, else pass list (unfiltered_list)
        //
        if list_nav(
            if let Some(list) = self.filtered_list.as_mut() {
                list
            }
            else {
                &mut self.list
            },
            key,
            &self.key_config
        ) {
            return Ok(EventState::Consumed);
        }

        // catch-all
        //
        Ok(EventState::NotConsumed)
    }
}

fn list_nav(list: &mut ProcessList, key: KeyEvent, key_config: &KeyConfig) -> bool {
    if let Some(common_nav) = common_nav(key, key_config) {
        list.move_selection(common_nav)
    }
    else {
        false
    }
}


impl StatefulDrawableComponent for CPUComponent {
    // TODO: Rewrite draw function
    //
    // TODO: Implement VerticalScroll?
    //
    // TODO: Move process_list_items.rs, process_list.rs, list_items_iter.rs, and list_iter.rs to Process Directory (add library).
    //
    fn draw(&mut self, f: &mut Frame, area: Rect) -> io::Result<()> {
        let window_height = area.height as usize;
        let list = self.get_process_list();
        let idx = self.get_idx();
        let pid = self.get_pid();
        let items:Vec<ListItem> =
        if pid.is_some() {
            list.iter()
            .skip(idx)
            .take(window_height)
            .map(|(p, n, c)| {
                let style = if *p == pid.unwrap() {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                }
                else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("PID: {:<10} Name: {:<50} CPU Usage: {:<15}", p, n, c))
                .style(style)
            })
            .collect::<Vec<_>>()
        }
        else {
            Vec::new()
        };
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Process List"))
            .style(Style::default().fg(Color::White));
        f.render_widget(list, area);
        Ok(())
    }
}