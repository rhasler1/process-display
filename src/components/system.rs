use std::io;
use crossterm::event::KeyEvent;
use sysinfo::{System, Pid};
use process_list::ProcessListItem;
use performance_queue::{CpuItem, MemoryItem};
use super::{Component, EventState};
use super::KeyConfig;

// See here for refreshing system: https://crates.io/crates/sysinfo#:~:text=use%20sysinfo%3A%3ASystem,(sysinfo%3A%3AMINIMUM_CPU_UPDATE_INTERVAL)%3B%0A%7D
// note: sysinfo::MINIMUM_CPU_UPDATE_INTERVAL = 200 ms

pub struct SystemComponent {
    system: System,
    //network: Networks,
    _key_config: KeyConfig
}

impl SystemComponent {
    pub fn new(key_config: KeyConfig) -> Self  {
        Self {
            system: System::new_all(),
            //network: Networks::new_with_refreshed_list(),
            _key_config: key_config,
        }
    }

    pub async fn refresh_all(&mut self) -> io::Result<EventState> {
        self.system.refresh_all(); // 1. refresh system
        //self.network.refresh_list(); // 2. refresh network interfaces list
        //self.network.refresh(); // 3. refresh network interfaces' content
        Ok(EventState::Consumed)
    }

    //pub fn get_network_info(&self) -> NetworkItem {

    //}

    pub fn get_cpu_info(&self) -> CpuItem {
        let mut brand_cpu: &str = "";
        let mut cpu_frequency: u64 = 0;
        for cpu in self.system.cpus() {
            brand_cpu = cpu.brand();
            cpu_frequency = cpu.frequency();
        }
        let global_cpu_usage = self.system.global_cpu_usage();
        let brand_cpu = String::from(brand_cpu);
        let num_cores = self.system.physical_core_count();
        let item = CpuItem::new(global_cpu_usage, num_cores, cpu_frequency, brand_cpu);
        item
    }

    pub fn get_memory_info(&self) -> MemoryItem {
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let free_memory = self.system.free_memory();
        let available_memory = self.system.available_memory();
        let memory_info = MemoryItem::new(total_memory, used_memory, free_memory, available_memory);
        memory_info
    }

    pub fn get_processes(&self) -> Vec<ProcessListItem> {
        let mut processes: Vec<ProcessListItem> = Vec::new();
        for (pid, process) in self.system.processes() {
            let name = self.get_process_name(*pid);
            let cpu_usage = process.cpu_usage();
            let memory_usage = process.memory();
            let item = ProcessListItem::new(pid.as_u32(), name, cpu_usage, memory_usage);
            processes.push(item);
        }
        return processes;
    }

    fn get_process_name(&self, pid: Pid) -> String {
        match self.system.process(pid) {
            //Some(p) => return String::from(p.name()),
            Some(p) => return String::from(p.name().to_str().unwrap_or_default()),
            None => return String::from("No Process given pid"),
        }
    }

    pub fn terminate_process(&mut self, pid: u32) -> io::Result<bool> {
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            process.kill();
        }
        Ok(true)
    }
}

impl Component for SystemComponent {
    fn event(&mut self, _key: KeyEvent) -> io::Result<EventState> {
        Ok(EventState::NotConsumed)
    }
}