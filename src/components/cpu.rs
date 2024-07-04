use std::io;
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use super::{filter::FilterComponent, Component, EventState, StatefulDrawableComponent};
use super::utils::vertical_scroll::VerticalScroll;

use crate::process::common_nav;
use crate::process::process_list_items::ProcessListItem;
use crate::process::process_list::ProcessList;
use crate::config::KeyConfig;

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
    scroll: VerticalScroll,
    key_config: KeyConfig,
}

impl CPUComponent {
    // default constructor
    //
    pub fn default() -> Self {
        Self {
            focus: Focus::List,
            list: ProcessList::default(),
            filter: FilterComponent::new(),
            filtered_list: None,
            scroll: VerticalScroll::new(false, false),
            key_config: KeyConfig::default(),
        }
    }
    
    // new custom constructor
    //
    pub fn new(list: &Vec<ProcessListItem>) -> Self {
        Self {
            focus: Focus::List,
            list: ProcessList::new(list),
            filter: FilterComponent::new(),
            filtered_list: None,
            scroll: VerticalScroll::new(false, false),
            key_config: KeyConfig::default(),
        }
    }

    // pub function to update the process list
    //
    pub async fn update(&mut self, new_processes: &Vec<ProcessListItem>) {
        self.list.update(new_processes);
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
        if matches!(self.focus, Focus::List) {
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
        }

        // catch-all
        //
        Ok(EventState::NotConsumed)
    }
}

fn list_nav(list: &mut ProcessList, key: KeyEvent, key_config: &KeyConfig) -> bool {
    if let Some(move_dir) = common_nav(key, key_config) {
        list.move_selection(move_dir)
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
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> io::Result<()> {
        // make chunks for list & filter
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // filter chunk
                Constraint::Min(1) // list chunk
            ].as_ref())
            .split(area);

        // draw filter
        self.filter.draw(f, chunks[0], matches!(self.focus, Focus::Filter))?;

        let list_height = chunks[1].height as usize;
        let list = if let Some(list) = self.filtered_list.as_ref() {
            list
        }
        else {
            &self.list
        };

        list.selection().map_or_else(
            { ||
                self.scroll.reset()
            }, |selection| {
                self.scroll.update(
                    selection, list_height
                );
            },
        );

        let items = list
            .iterate(self.scroll.get_top(), list_height)
            .map(|(item, selected)| {
                let style =
                if selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                }
                else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("PID: {}, Name: {}, Selected: {:?}", item.pid(), item.name(), list.selection))
                    .style(style)
            })
            .collect::<Vec<_>>();

        let drawable_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Process List"))
            .style(Style::default().fg(Color::White));

        f.render_widget(drawable_list, chunks[1]);

        Ok(())
    }
}