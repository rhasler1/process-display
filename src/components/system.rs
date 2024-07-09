use std::io;
use crossterm::event::{KeyEvent, KeyCode};
use sysinfo::{System, Pid};


use super::{Component, EventState};

use crate::process::process_list_items::CpuInfo;
use crate::process::process_list_items::ProcessListItem;


// I want to asynch refresh the cpu
// refreshing cpu see here: https://crates.io/crates/sysinfo#:~:text=use%20sysinfo%3A%3ASystem,(sysinfo%3A%3AMINIMUM_CPU_UPDATE_INTERVAL)%3B%0A%7D
// asynch
// note: sysinfo::MINIMUM_CPU_UPDATE_INTERVAL = 200 ms

pub struct SystemWrapper {
    // process_list[0] == PID, process_list[1] == process_name, process_list[2] == cpu_usage
    cpu_process_list: Vec<ProcessListItem>, // main data structure list of processes
    system: System, // system
}

impl SystemWrapper {
    pub fn new() -> Self  {
        Self {
            cpu_process_list: Vec::new(),
            system: System::new_all(),
        }
    }

    // pub method refresh_all
    // 1. clears old data from process_list
    // 2. refreshes all fields in the internal system structure
    // 3. sets the process list with new data from
    //
    pub fn refresh_all(&mut self) -> io::Result<EventState> {
        self.cpu_process_list.clear(); // 1. clear process list
        self.system.refresh_all(); // 2. refresh system
        self.set_cpu_process_list(); // 3. set the process list
        Ok(EventState::Consumed)
    }

    // fn set_cpu_process_list
    //
    fn set_cpu_process_list(&mut self) {
        for (pid, process) in self.system.processes() {
            // get process name and cpu usage
            //
            let name = self.get_process_name(*pid);
            let cpu_usage = process.cpu_usage();

            // instantiate ProcessListItem
            //
            let cpu_info = CpuInfo::new(pid.as_u32(), name, cpu_usage);
            let item = ProcessListItem::Cpu(cpu_info);

            // push item on list
            //
            self.cpu_process_list.push(item);
        }
    }

    // method get_process_name
    // inputs:
    //   pid: Pid -- A PID to retrieve a process name from
    // outputs:
    //   String -- The process name belonging to pid
    //
    fn get_process_name(&self, pid: Pid) -> String {
        match self.system.process(pid) {
            Some(p) => return String::from(p.name()),
            None => return String::from("No Process given pid"),
        }
    }

    pub fn terminate_process(&mut self, pid: u32) -> io::Result<bool> {
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            process.kill();
        }
        Ok(true)
    }

    // pub method get_cpu_process_list
    // outputs:
    //   cpu_process_list: Vec<(u32, String, f32)> -- Vector containing information relating processes and cpu usage
    pub fn get_cpu_process_list(&mut self) -> &Vec<ProcessListItem> {
        return self.cpu_process_list.as_ref();
    }
}

impl Component for SystemWrapper {
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if key.code == KeyCode::Char('r') {
            self.refresh_all()?;
            return Ok(EventState::Consumed)
        }
        return Ok(EventState::NotConsumed)
    }
}