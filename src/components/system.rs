use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};
use sysinfo::{System, Pid};

use super::StatefulDrawableComponent;
use super::Component;

pub enum ProcessFocus {
    Unfiltered,
    Filtered,
}

pub struct SystemWrapper {
    focus: ProcessFocus,
    pid_in_focus_idx: usize, // index of pid_in_focus
    pid_in_focus: u32, // u32 value of pid in focus
    process_list: Vec<(u32, String)>, // main data structure list of processes
    filtered_process_list: Option<Vec<(u32, String)>>, // implement for search function
    system: System, // system
}

impl SystemWrapper {
    pub fn new() -> Self  {
        Self {
            focus: ProcessFocus::Unfiltered,
            pid_in_focus_idx: 0,
            pid_in_focus: 0,
            process_list: Vec::new(),
            filtered_process_list: None,
            system: System::new_all(),
        }
    }

    // method reset can also be used to `set` the system
    pub fn reset(&mut self) {
        self.process_list.clear(); // 1. clear process list
        self.system.refresh_all(); // 2. refresh system
        self.set_process_list(); // 3. set the process list
        self.set_pid_in_focus(); // 4. init focus -> must only be called when process list is set(populated)
    }

    fn move_focus_filtered(&mut self) {
        self.focus = ProcessFocus::Filtered;
    }

    fn move_focus_unfiltered(&mut self) {
        self.focus = ProcessFocus::Unfiltered;
    }

    fn terminate_process(&mut self) -> io::Result<bool> {
        if let Some(process) = self.system.process(Pid::from_u32(self.pid_in_focus)) {
            process.kill();
        }
        Ok(true)
    }

    // pub fn sort() // TODO: write generic sort using enums (ie: PidInc, PidDec, NameInc, NameDec, ...)
    //fn sort_by_pid(&mut self) {
    //    self.process_list.sort_by_key(|k| k.0);
    //}   

    fn set_process_list(&mut self) {
        for (pid, _process) in self.system.processes() {
            let process_name = self.get_process_name(*pid);
            self.process_list.push((pid.as_u32(), process_name));
        }
    }

    fn get_process_name(&self, pid: Pid) -> String {
        match self.system.process(pid) {
            Some(p) => return String::from(p.name()),
            None => return String::from("No Process given pid"),
        }
    }

    // init pid in focus
    fn set_pid_in_focus(&mut self) {
        self.pid_in_focus_idx = 0;
        if self.process_list.is_empty() {
            return;
        }
        self.pid_in_focus = self.process_list[self.pid_in_focus_idx].0;
    }

    // increment focus and set pid in focus
    // using circular indexing of process_list
    fn inc_focus(&mut self) -> io::Result<bool> {
        if self.process_list.is_empty() {
            return Ok(false);
        }
        self.pid_in_focus_idx = (self.pid_in_focus_idx + 1) % self.process_list.len();
        self.pid_in_focus = self.process_list[self.pid_in_focus_idx].0;
        return Ok(true);
    }

    // decrement focus and set pid in focus
    // using circular indexing of process_list
    fn dec_focus(&mut self) -> io::Result<bool> {
        if self.process_list.is_empty() {
            return Ok(false);
        }
        if self.pid_in_focus_idx == 0 {
            self.pid_in_focus_idx = self.process_list.len() - 1;
            self.pid_in_focus = self.process_list[self.pid_in_focus_idx].0;
            return Ok(true)
        }
        self.pid_in_focus_idx = (self.pid_in_focus_idx - 1) % self.process_list.len();
        self.pid_in_focus = self.process_list[self.pid_in_focus_idx].0;
        return Ok(true);
    }
}

impl Component for SystemWrapper {
    fn event(&mut self, key: KeyEvent, filter: Option<String>) -> io::Result<bool> {
        // update focus::begin
        if filter.is_some() {
            self.move_focus_filtered();
            filter.unwrap();
        }
        else {
            self.move_focus_unfiltered();
        }
        // update focus::end

        if key.code == KeyCode::Char('r') {
            self.reset();
            return Ok(true)
        }
        if key.code == KeyCode::Down {
            // moving down the list means focus must be incremented
            self.inc_focus()?;
            return Ok(true)
        }
        if key.code == KeyCode::Up {
            // moving up the list means focus must be decremented
            self.dec_focus()?;
            return Ok(true)
        }
        if key.code == KeyCode::Char('t') {
            // terminate the current process in focus
            self.terminate_process()?;
            // for now, resetting the system after terminating a process, this is
            // very inefficient -- will improve later.
            self.reset();
            return Ok(true)
        }
        return Ok(false)
    }
}

impl StatefulDrawableComponent for SystemWrapper {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> io::Result<bool> {
        let window_height = area.height as usize;

        let items: Vec<ListItem> = self.process_list.iter()
            .skip(self.pid_in_focus_idx)
            .take(window_height)
            .map(|(pid, name)| {
                let style = if *pid == self.pid_in_focus {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                }
                else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("PID: {}, Name: {}", pid, name))
                    .style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Process List"))
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
        Ok(true)
    }
}