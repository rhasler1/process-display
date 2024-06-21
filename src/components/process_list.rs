use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

use super::StatefulDrawableComponent;
use super::Component;

pub enum FilterState {
    NotFiltering,
    Filtering,
}

pub struct ProcessList {
    filter_state: FilterState,
    filter_name: String,
    filtered_list: Vec<(u32, String)>,
    unfiltered_list: Vec<(u32, String)>,
    filtered_idx: usize,
    unfiltered_idx: usize,
}

impl ProcessList {
    pub fn new() -> Self {
        Self {
            filter_state: FilterState::NotFiltering,
            filter_name: String::new(),
            filtered_list: Vec::new(),
            unfiltered_list: Vec::new(),
            filtered_idx: 0,
            unfiltered_idx: 0,
        }
    }

    pub fn reset(&mut self) {
        self.filter_state = FilterState::NotFiltering;
        self.filter_name.clear();
        self.filtered_list.clear();
        self.unfiltered_list.clear();
        self.filtered_idx = 0;
        self.unfiltered_idx = 0;        
    }

    pub fn set_filter_name(&mut self, n: String) {
        self.filter_name = n.clone();
    }

    pub fn set_filtered_list(&mut self) {
        let temp_list = self.unfiltered_list.clone();
        self.filtered_list = temp_list.into_iter().filter(|(_, name)| &self.filter_name == name).collect();
    }

    pub fn set_unfiltered_list(&mut self, list: Vec<(u32, String)>) {
        self.unfiltered_list = list.clone();
    }

    pub fn get_idx(&mut self) -> usize {
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

    pub fn get_process_list(&mut self) -> Vec<(u32, String)> {
        match self.filter_state {
            FilterState::NotFiltering => {
                return self.unfiltered_list.clone();
            }
            FilterState::Filtering => {
                return self.filtered_list.clone();
            }
        }
    }

    // function is not safe! will panic if list length is 0.
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

impl Component for ProcessList {
    fn event(&mut self, key: KeyEvent) -> io::Result<bool> {
        match key.code {
            KeyCode::Up => {
                self.dec_idx();
                return Ok(true);
            }
            KeyCode::Down => {
                self.inc_idx();
                return Ok(true);
            }
            KeyCode::Enter => {
                self.swap_filter();
                return Ok(true);
            }
            _=> { return Ok(false) }
        }
    }
}

impl StatefulDrawableComponent for ProcessList {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> io::Result<bool> {
        let window_height = area.height as usize;
        let list = self.get_process_list();
        let idx = self.get_idx();
        let pid = self.get_pid();

        let items:Vec<ListItem> = if pid.is_some() {
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
        Ok(true)
    }
}