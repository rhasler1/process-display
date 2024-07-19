use std::io;
use crossterm::event::KeyEvent;
use sysinfo::{System, Pid};
use super::{Component, EventState};
use crate::{config::KeyConfig, performance_structs::perf_item::CpuItem, process_structs::process_list_item::ProcessListItem};


// See here for refreshing system: https://crates.io/crates/sysinfo#:~:text=use%20sysinfo%3A%3ASystem,(sysinfo%3A%3AMINIMUM_CPU_UPDATE_INTERVAL)%3B%0A%7D
// note: sysinfo::MINIMUM_CPU_UPDATE_INTERVAL = 200 ms

pub struct SystemComponent {
    system: System,
    process_list: Vec<ProcessListItem>,
    key_config: KeyConfig
}

impl SystemComponent {
    pub fn new(key_config: KeyConfig) -> Self  {
        Self {
            system: System::new_all(),
            process_list: Vec::new(),
            key_config: key_config,
        }
    }

    pub async fn refresh_all(&mut self) -> io::Result<EventState> {
        self.process_list.clear(); // 1. clear process list
        self.system.refresh_all(); // 2. refresh system
        self.set_process_list(); // 3. set the process list
        Ok(EventState::Consumed)
    }

    fn set_process_list(&mut self) {
        for (pid, process) in self.system.processes() {
            let name = self.get_process_name(*pid);
            let cpu_usage = process.cpu_usage();
            let memory_usage = process.memory();
            let item = ProcessListItem::new(pid.as_u32(), name, cpu_usage, memory_usage);
            self.process_list.push(item);
        }
    }

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

    pub fn get_process_list(&self) -> &Vec<ProcessListItem> {
        return self.process_list.as_ref();
    }

    pub fn get_cpu_info(&self) -> CpuItem {
        let mut total_cpu_usage = 0.0;
        let mut brand_cpu: &str = "";
        let mut cpu_frequency: u64 = 0;
        for cpu in self.system.cpus() {
            total_cpu_usage += cpu.cpu_usage();
            brand_cpu = cpu.brand();
            cpu_frequency = cpu.frequency();
        }
        let brand_cpu = String::from(brand_cpu);
        let num_cores = self.system.physical_core_count();
        let item = CpuItem::new(total_cpu_usage, num_cores, cpu_frequency, brand_cpu);
        item
    }
}

impl Component for SystemComponent {
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        if key.code == self.key_config.terminate {
            return Ok(EventState::NotConsumed)
        }
        Ok(EventState::NotConsumed)
    }
}