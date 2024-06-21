use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use sysinfo::{System, Pid};

use super::Component;

pub struct SystemWrapper {
    process_list: Vec<(u32, String)>, // main data structure list of processes
    system: System, // system
}

impl SystemWrapper {
    pub fn new() -> Self  {
        Self {
            process_list: Vec::new(),
            system: System::new_all(),
        }
    }

    // method reset can also be used to `set` the system
    pub fn reset(&mut self) {
        self.process_list.clear(); // 1. clear process list
        self.system.refresh_all(); // 2. refresh system
        self.set_process_list(); // 3. set the process list
    }

    pub fn terminate_process(&mut self, pid: u32) -> io::Result<bool> {
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            process.kill();
        }
        Ok(true)
    }

    // pub fn sort() // TODO: write generic sort using enums (ie: PidInc, PidDec, NameInc, NameDec, ...)
    //fn sort_by_pid(&mut self) {
    //    self.process_list.sort_by_key(|k| k.0);
    //}

    pub fn get_process_list(&mut self) -> Vec<(u32, String)> {
        return self.process_list.clone();
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
}

impl Component for SystemWrapper {
    fn event(&mut self, key: KeyEvent) -> io::Result<bool> {
        if key.code == KeyCode::Char('r') {
            self.reset();
            return Ok(true)
        }
        return Ok(false)
    }
}