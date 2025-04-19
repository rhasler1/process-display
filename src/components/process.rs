use anyhow::{Ok, Result};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    prelude::*,
};
use process_list::{ProcessList, ProcessListItem, ProcessListItems};

use super::{common_nav, common_sort};
use super::{filter::FilterComponent, Component, DrawableComponent, EventState};
use super::utils::vertical_scroll::VerticalScroll;
use crate::config::Config;
use crate::config::KeyConfig;
use crate::ui::process_list_ui::process_list_ui::draw_process_list;

// focus of process component, this can be on either a ProcessList or FilterComponent
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
    // constructor
    pub fn new(config: Config) -> Self {
        Self {
            focus: Focus::List,
            list: ProcessList::default(),
            filter: FilterComponent::new(config.clone()),
            filtered_list: None,
            scroll: VerticalScroll::new(),
            config: config,
        }
    }

    // update process list, return true if new processes is non empty
    pub fn update(&mut self, new_processes: &Vec<ProcessListItem>) -> bool {
        if new_processes.is_empty() {
            return false
        }
        
        self.list.update(new_processes);   

        if let Some(filtered_list) = self.filtered_list.as_mut() {
            // filter new processes
            let processes = ProcessListItems::new(new_processes);
            let filter_text = self.filter.input_str();
            let filtered_processes = processes.filter(&filter_text);
            filtered_list.update(&filtered_processes.list_items);
        }
        true
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
    // Handle key events for ProcessComponent.
    fn event(&mut self, key: KeyEvent) -> Result<EventState> {
        //  If they key event is filter and the ProcessComponent Focus is on the List, then move the focus to Filter and return.
        if key.code == self.config.key_config.filter && self.focus == Focus::List {
            self.focus = Focus::Filter;
            return Ok(EventState::Consumed)
        }

        // If the ProcessComponent Focus is on the Filter, then attempt to set the filtered_list.
        // If the filter's input string is None, then set the filtered_list to None (no List to display),
        // else create the filtered_list calling list.filter(input_str).
        if matches!(self.focus, Focus::Filter) {
            self.filtered_list = if self.filter.input_str().is_empty() {
                None
            }
            else {
                Some(self.list.filter(&self.filter.input_str()))
            };
        }

        // If the key event is enter and the focus is on the Filter, then change the focus to List and return.
        if key.code == self.config.key_config.enter && matches!(self.focus, Focus::Filter) {
            self.focus = Focus::List;
            return Ok(EventState::Consumed)
        }

        // The following if block contains key event tasks for when the filter is in focus.
        if matches!(self.focus, Focus::Filter) {
            // Check if the key input is to modify the filter.
            if self.filter.event(key)?.is_consumed() {
                return Ok(EventState::Consumed)
            }
        }

        //  The following if block contains key event tasks for when the process list is in focus.
        if matches!(self.focus, Focus::List) {
            // Check if the key input is to navigate the list. If there is some filtered list, then that is the
            // list we want to interact with, else interact with the unfiltered list.
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

            // Check if the key is to change the follow_selection value.
            else if key.code == self.config.key_config.follow_selection {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.change_follow_selection();
                }
                else {
                    self.list.change_follow_selection();
                }

                return Ok(EventState::Consumed)
            }

            // Check if the key input is to sort the list. If there is some filtered list, then that is the
            // list we want to interact with, else interact with unfiltered list.
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


// Function calls common_nav, common_nav checks if key can be consumed, if so,
// Some(MoveSelection) is returned and list.move_selection(MoveSelection) is called.
// Else return false.
fn list_nav(list: &mut ProcessList, key: KeyEvent, key_config: &KeyConfig) -> bool {
    if let Some(move_dir) = common_nav(key, key_config) {
        list.move_selection(move_dir);
        true
    }
    else {
        false
    }
}

// Function calls common_sort, common_sort checks if key can be consumed, if so,
// Some(ListSortOrder) is returned and list.sort(ListSortOrder) is called.
// Else return false.
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
        // split for filter
        let horizontal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(3),
            ]).split(area);

        // calculate visible list height
        let visible_list_height = horizontal_chunks[0].height.saturating_sub(3) as usize;

        // determine list to display
        let list = if let Some(filtered_list) = self.filtered_list.as_ref() {
            filtered_list
        }
        else {
            &self.list
        };

        // updating the scroll struct--calculates the position at the top of the displayed list
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
            self.list.is_follow_selection(),
            if focused {
                matches!(self.focus, Focus::List)
            } 
            else {
                false
            },
            self.config.theme_config.clone()
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

#[cfg(test)]
mod test {
    // TODO: write tests
}