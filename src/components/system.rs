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

pub struct SystemWrapper {
    focus: usize, // index of pid_in_focus
    pid_in_focus: u32, // u32 value of pid in focus
    system: System, // system
    process_list: Vec<(u32, String)>, // main data structure list of processes
}

impl SystemWrapper {
    pub fn new() -> Self  {
        Self {
            focus: 0,
            pid_in_focus: 0,
            system: System::new_all(),
            process_list: Vec::new(),
        }
    }

    // method reset can also be used to `set` the system
    pub fn reset(&mut self) -> io::Result<bool> {
        self.process_list.clear(); // 1. clear process list
        self.system.refresh_all(); // 2. refresh system
        self.set_process_list(); // 3. set the process list
        self.init_focus()?; // 4. init focus -> must only be called when process list is set(populated)
        return Ok(true)
    }

    pub fn terminate_process(&mut self) -> io::Result<bool> {
        //TODO: implement
        if let Some(process) = self.system.process(Pid::from_u32(self.pid_in_focus)) {
            process.kill();
        }
        Ok(true)
    }

    // pub fn sort() // TODO: write generic sort using enums (ie: PidInc, PidDec, NameInc, NameDec, ...)
    pub fn sort_by_pid(&mut self) {
        self.process_list.sort_by_key(|k| k.0);
    }   

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
    fn init_focus(&mut self) -> io::Result<bool> {
        self.focus = 0;
        if self.process_list.is_empty() {
            return Ok(false);
        }
        self.pid_in_focus = self.process_list[self.focus].0;
        return Ok(true);
    }

    // increment focus and set pid in focus
    // using circular indexing of process_list
    fn inc_focus(&mut self) -> io::Result<bool> {
        if self.process_list.is_empty() {
            return Ok(false);
        }
        self.focus = (self.focus + 1) % self.process_list.len();
        self.pid_in_focus = self.process_list[self.focus].0;
        return Ok(true);
    }

    // decrement focus and set pid in focus
    // using circular indexing of process_list
    fn dec_focus(&mut self) -> io::Result<bool> {
        if self.process_list.is_empty() {
            return Ok(false);
        }
        self.focus = (self.focus - 1) % self.process_list.len();
        self.pid_in_focus = self.process_list[self.focus].0;
        return Ok(true);
    }
}

impl Component for SystemWrapper {
    fn event(&mut self, key: KeyEvent) -> io::Result<bool> {
        if key.code == KeyCode::Char('r') {
            self.reset()?;
            return Ok(true)
        }
        if key.code == KeyCode::Down {
            // moving down the list means focus must be incremented
            self.inc_focus()?;
        }
        if key.code == KeyCode::Up {
            // moving up the list means focus must be decremented
            self.dec_focus()?;
        }
        if key.code == KeyCode::Char('t') {
            // terminate the current process in focus
            self.terminate_process()?;
            // for now, resetting the system after terminating a process, this is
            // very inefficient -- will improve later.
            self.reset()?;
        }
        return Ok(false)
    }
}

impl StatefulDrawableComponent for SystemWrapper {
    fn draw(&mut self, f: &mut Frame, area: Rect) -> io::Result<bool> {
        let items: Vec<ListItem> = self.process_list.iter()
            .map(|(pid, name)| {
                ListItem::new(format!("PID: {}, Name: {}", pid, name))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Process List"))
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
        Ok(true)
    }
}