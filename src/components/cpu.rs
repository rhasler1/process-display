use std::io;
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use super::{EventState, StatefulDrawableComponent, Component};
use crate::config::KeyConfig;

// CPUComponent can be in one of two states-- NotFiltering or Filtering. This enumerator
// is implemented so the methods in CPUComponent interact with only the data structure
// that corresponds to the `current` FilterState (ie: NotFiltering => unfiltered_list: Vec<(u32, String)>).
pub enum FilterState {
    NotFiltering,
    Filtering,
}

// CPUComponent is an observer of SystemWrapper, storing both unfiltered and filtered process
// lists of the system. An action like process termination must be handled by SystemWrapper.
// In the case that the user wishes to termiante a process, the CPU component provides the
// SystemWrapper with the PID of the process to termiante (this communcation happens in the
// impl of App).
pub struct CPUComponent {
    filter_state: FilterState,
    filter_name: String, // filtering is done by process name
    filtered_list: Vec<(u32, String)>,
    unfiltered_list: Vec<(u32, String)>,
    filtered_idx: usize,
    unfiltered_idx: usize,
    key_config: KeyConfig,
}

impl CPUComponent {
    // pub method to construct CPUComponent
    //
    pub fn new() -> Self {
        Self {
            filter_state: FilterState::NotFiltering,
            filter_name: String::new(),
            filtered_list: Vec::new(),
            unfiltered_list: Vec::new(),
            filtered_idx: 0,
            unfiltered_idx: 0,
            key_config: KeyConfig::default(),
        }
    }

    // pub method to reset all struct fields
    //
    pub fn reset(&mut self) {
        self.filter_state = FilterState::NotFiltering;
        self.filter_name.clear();
        self.filtered_list.clear();
        self.unfiltered_list.clear();
        self.filtered_idx = 0;
        self.unfiltered_idx = 0;        
    }

    // public method to set CPUComponent.unfiltered_list
    // inputs:
    //   list: Vec<()> -- A list of PID's and process information having to do with the CPU.
    //
    pub fn set_unfiltered_list(&mut self, list: Vec<(u32, String)>) {
        self.unfiltered_list = list.clone();
    }

    // public method to set CPUComponent.filtered_list
    // inputs:
    //   n: String -- A process name to filter CPUComponent.unfiltered_list.
    //
    pub fn set_filtered_list(&mut self, n: String) {
        self.filter_name.clear();
        self.filter_name = n.clone();
        let temp_list = self.unfiltered_list.clone();
        self.filtered_list = temp_list.into_iter().filter(|(_, name)| &self.filter_name == name).collect();
    }

    // public method to get the current pid of either the unfiltered_list or filtered_list
    // returns:
    //   Some<u32> || None
    //
    pub fn get_pid(&mut self) -> Option<u32> {
        match self.filter_state {
            FilterState::NotFiltering => {
                if self.unfiltered_list.len() < 1 {
                    return None
                }
                else {
                    return Some(self.unfiltered_list[self.unfiltered_idx].0);
                }
            }
            FilterState::Filtering => {
                if self.filtered_list.len() < 1 {
                    return None
                }
                else {
                    return Some(self.filtered_list[self.filtered_idx].0);
                }
            }
        }
    }

    // pub method to change the value of self.filter_state
    //
    pub fn swap_filter(&mut self) {
        match self.filter_state {
            FilterState::NotFiltering => {
                self.filter_state = FilterState::Filtering;
            }
            FilterState::Filtering => {
                self.filter_state = FilterState::NotFiltering;
            }
        }
    }

    // method to get either the unfiltered_list or filtered_list
    // returns:
    //   list: Vec<()> -- The value of filter_state determined which list to return.
    //
    fn get_process_list(&mut self) -> Vec<(u32, String)> {
        match self.filter_state {
            FilterState::NotFiltering => {
                return self.unfiltered_list.clone();
            }
            FilterState::Filtering => {
                return self.filtered_list.clone();
            }
        }
    }

    // method to get the current index of either the unfiltered_list or filtered_list
    // returns:
    //  idx: usize -- The value of filter_state determines which index to return.
    //
    fn get_idx(&mut self) -> usize {
        let idx = match self.filter_state {
            FilterState::NotFiltering => {
                self.unfiltered_idx
            }
            FilterState::Filtering => {
                self.filtered_idx
            }
        };
        return idx;
    }

    // method to inc the index of either the unfiltered_list or filtered_list
    //
    fn inc_idx(&mut self) {
        match self.filter_state {
            FilterState::NotFiltering => {
                if self.unfiltered_list.is_empty() {
                    return
                }
                self.unfiltered_idx = (self.unfiltered_idx + 1) % self.unfiltered_list.len();
                return
            }
            FilterState::Filtering => {
                if self.filtered_list.is_empty() {
                    return
                }
                self.filtered_idx = (self.filtered_idx + 1) % self.filtered_list.len();
                return
            }
        }
    }

    // method to dec the index of either the unfiltered_list or filtered_list
    //
    fn dec_idx(&mut self) {
        match self.filter_state {
            FilterState::NotFiltering => {
                if self.unfiltered_list.is_empty() {
                    return
                }
                if self.unfiltered_idx == 0 {
                    self.unfiltered_idx = self.unfiltered_list.len() - 1;
                    return
                }
                self.unfiltered_idx = (self.unfiltered_idx - 1) % self.unfiltered_list.len();
                return
            }
            FilterState::Filtering => {
                if self.filtered_list.is_empty() {
                    return
                }
                if self.filtered_idx == 0 {
                    self.filtered_idx = self.filtered_list.len() - 1;
                    return
                }
                self.filtered_idx = (self.filtered_idx - 1) % self.filtered_list.len();
                return
            }
        }    
    }
}

impl Component for CPUComponent {
    // handle key events for CPUComponent
    //
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if key.code == self.key_config.move_up {
            self.dec_idx();
            return Ok(EventState::Consumed);
        }
        if key.code == self.key_config.move_down {
            self.inc_idx();
            return Ok(EventState::Consumed);
        }
        if key.code == self.key_config.filter {
            self.swap_filter();
            return Ok(EventState::Consumed);
        }
        return Ok(EventState::NotConsumed);
    }
}

impl StatefulDrawableComponent for CPUComponent {
    // draw the current state of CPUComponent -- drawing list and highlighting entry of the index in `focus`
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
            .map(|(p, n)| {
                let style = if *p == pid.unwrap() {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                }
                else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("PID: {}, Name: {}", p, n))
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