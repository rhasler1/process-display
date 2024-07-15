use std::io;
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};
use super::{filter::FilterComponent, Component, EventState, ListSortOrder, StatefulDrawableComponent};
use super::utils::vertical_scroll::VerticalScroll;
use crate::process::{common_nav, process_list_items::ProcessListItems};
use crate::process::process_list_item::ProcessListItem;
use crate::process::process_list::ProcessList;
use crate::config::KeyConfig;

// The CPU Component can be navigated to focus on
// either a ProcessList <filtered/unfiltered> or
// FilterComponent.
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
    
    // new constructor
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
    pub async fn update(&mut self, new_processes: &Vec<ProcessListItem>) -> io::Result<()> {
        // update list
        self.list.update(new_processes)?;
        // update filter list
        if let Some(filtered_list) = self.filtered_list.as_mut() {
            let processes = ProcessListItems::new(new_processes);
            let filter_text = self.filter.input_str();
            let filtered_processes = processes.filter(filter_text);
            filtered_list.update(&filtered_processes.list_items)?;
        }
        Ok(())
    }

    //  pub fn list -- getter
    pub fn list(&self) -> &ProcessList {
        self.filtered_list.as_ref().unwrap_or(&self.list)
    }

    // pub fn list_focused -- getter
    pub fn list_focused(&self) -> bool {
        matches!(self.focus, Focus::List)
    }
}

impl Component for CPUComponent {
    // handle key events for CPUComponent
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        //  If they key event is filter and the CPUComponent Focus is on the List, then move the focus to Filter and return.
        if key.code == self.key_config.filter && self.focus == Focus::List {
            self.focus = Focus::Filter;
            return Ok(EventState::Consumed)
        }

        // if the CPUComponent Focus is on the Filter, then attempt to set the filtered_list.
        // if the filter's input string is None, then set the filtered_list to None (no List to display),
        // else create the filtered_list calling list.filter(input_str)
        if matches!(self.focus, Focus::Filter) {
            self.filtered_list = if self.filter.input_str().is_empty() {
                None
            }
            else {
                Some(self.list.filter(self.filter.input_str()))
            };
        }

        // if the key event is enter and the focus is Filter, then change the focus to List and return.
        if key.code == self.key_config.enter && matches!(self.focus, Focus::Filter) {
            self.focus = Focus::List;
            return Ok(EventState::Consumed)
        }

        // if the focus is Filter
        // pass the key event to self.filter and attempt to consume.
        if matches!(self.focus, Focus::Filter) {
            if self.filter.event(key)?.is_consumed() {
                return Ok(EventState::Consumed)
            }
        }

        //  if the filtered_list is Some pass it as argument, else pass list (unfiltered_list)
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

            // check if key code is follow selection
            else if key.code == self.key_config.follow_selection {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.change_follow_selection()?;
                }
                else {
                    self.list.change_follow_selection()?;
                }

                return Ok(EventState::Consumed);
            }

            // check different sort options
            else if key.code == self.key_config.sort_name_inc {
                // if there is some filtered_list sort the filtered list
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.sort(ListSortOrder::NameInc)?;
                }
                else {
                    self.list.sort(ListSortOrder::NameInc)?;
                }
                return Ok(EventState::Consumed);
            }

            else if key.code == self.key_config.sort_name_dec {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.sort(ListSortOrder::NameDec)?;
                }
                else {
                    self.list.sort(ListSortOrder::NameDec)?;
                }
                return Ok(EventState::Consumed)
            }

            else if key.code == self.key_config.sort_pid_inc {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.sort(ListSortOrder::PidInc)?;
                }
                else {
                    self.list.sort(ListSortOrder::PidInc)?;
                }
                return Ok(EventState::Consumed);
            }

            else if key.code == self.key_config.sort_pid_dec {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.sort(ListSortOrder::PidDec)?;
                }
                else {
                    self.list.sort(ListSortOrder::PidDec)?;
                }
                return Ok(EventState::Consumed);
            }

            else if key.code == self.key_config.sort_usage_inc {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.sort(ListSortOrder::UsageInc)?;
                }
                else {
                    self.list.sort(ListSortOrder::UsageInc)?;
                }
                return Ok(EventState::Consumed);
            }

            else if key.code == self.key_config.sort_usage_dec {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.sort(ListSortOrder::UsageDec)?;
                }
                else {
                    self.list.sort(ListSortOrder::UsageDec)?;
                }
                return Ok(EventState::Consumed);
            }
        }
        
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
    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> io::Result<()> {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // filter chunk
                Constraint::Min(1), // list chunk
            ].as_ref())
            .split(area);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // space for tab
                Constraint::Percentage(50), // space for filter
            ].as_ref())
            .split(vertical_chunks[0]);

        self.filter.draw(f, horizontal_chunks[1], matches!(self.focus, Focus::Filter))?;

        // note: saturating sub 2 to account for drawing the block border see variable drawable_list
        let list_height = (vertical_chunks[1].height.saturating_sub(2)) as usize;

        // get list to display if Some(filtered_list) set list to filtered_list else set to unfiltered list
        let list = if let Some(list) = self.filtered_list.as_ref() {
            list
        }
        else {
            &self.list
        };

        // update the scroll struct-- determines what indices of the list are displayed
        list.selection().map_or_else(
            { ||
                self.scroll.reset()
            }, |selection| {
                self.scroll.update(
                    selection, list_height
                );
            },
        );

        // get list.follow() to visually differentiate between a selected item being followed(underlined) and not
        let follow_flag = list.follow();

        let items = list
            .iterate(self.scroll.get_top(), list_height)
            .map(|(item, selected)| {
                let style =
                    if matches!(self.focus, Focus::List) && selected && follow_flag {
                        Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED)
                    }
                    else if matches!(self.focus, Focus::List) && selected && !follow_flag {
                        Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD)
                    }
                    else if matches!(self.focus, Focus::List) {
                        Style::default().fg(Color::White)
                    }
                    else {
                        Style::default().fg(Color::DarkGray)
                    };
                    ListItem::new(
                        format!(
                            "PID: {:<5} Name: {:<50} Cpu Usage: {:?}%", // widths
                            item.pid(),
                            item.name(),
                            item.cpu_usage().unwrap()
                        )
                    )
                    .style(style)
                })
                .collect::<Vec<_>>();

        let block_style =
            if matches!(self.focus, Focus::List) {
                Style::default().fg(Color::White)
            }
            else {
                Style::default().fg(Color::DarkGray)
            };
        
        let block_title: &str = "Process List";

        let drawable_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(block_title))
            .style(block_style);

        f.render_widget(drawable_list, vertical_chunks[1]);

        Ok(())
    }
}