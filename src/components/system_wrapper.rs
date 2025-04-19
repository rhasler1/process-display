use std::vec;
use sysinfo::*;
use anyhow::Result;
use crossterm::event::KeyEvent;
use sysinfo::{Cpu, Pid, System};
use process_list::ProcessListItem;
use performance_queue::{CpuItem, MemoryItem};
use crate::config::Config;
use super::{Component, EventState};
use process_list::ProcessItemInfo;

// See here for refreshing system: https://crates.io/crates/sysinfo#:~:text=use%20sysinfo%3A%3ASystem,(sysinfo%3A%3AMINIMUM_CPU_UPDATE_INTERVAL)%3B%0A%7D
// note: sysinfo::MINIMUM_CPU_UPDATE_INTERVAL = 200 ms

pub struct SystemWrapper {
    system: System,
    pub _config: Config
}

impl SystemWrapper {
    pub fn new(config: Config) -> Self  {
        Self {
            system: System::new_all(),
            _config: config
        }
    }

    pub fn refresh_all(&mut self) -> Result<EventState> {
        self.system.refresh_all();
        
        Ok(EventState::Consumed)
    }

    pub fn get_cpus(&self) -> Vec<CpuItem> {
        let mut cpus: Vec<CpuItem> = Vec::new();

        // dummy item for global cpu usage
        cpus.push(CpuItem::new(
            0,
            self.system.global_cpu_usage(),
            0, String::from("Global"),
            String::from(""),
            String::from("")
        ));

        for (id, cpu) in self.system.cpus().iter().enumerate() {
            let cpu_item = CpuItem::new(
                id + 1, // id=0 reserved for global cpu usage                   
                cpu.cpu_usage(),
                cpu.frequency(),
                String::from(cpu.name()),
                String::from(cpu.brand()),
                String::from(cpu.vendor_id()),
            );

            cpus.push(cpu_item);
        }

        cpus
    }

    pub fn get_static_sysinfo() -> Vec<String> {
        let vec: Vec<String> = vec![
            sysinfo::System::kernel_long_version(),
            if let Some(name) = sysinfo::System::host_name() {
                name
            }
            else {
                String::from("None")
            },
            sysinfo::System::cpu_arch(),
            if let Some(count) = sysinfo::System::physical_core_count() {
                format!("{count}")
            }
            else {
                String::from("None")
            },
            if let Some(version) = sysinfo::System::long_os_version() {
                version
            }
            else {
                String::from("None")
            },
            format!("{}", sysinfo::System::uptime())
        ];

        vec
    }

    pub fn update_sys_info() -> String {
        format!("{}", sysinfo::System::uptime())
    }

    pub fn get_global_cpu_info(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    pub fn get_processes(&self) -> Vec<ProcessListItem> {
        let mut processes: Vec<ProcessListItem> = Vec::new();

        for (pid, process) in self.system.processes() {
            let name = if let Some(name) = process.name().to_str() {
                String::from(name)
            }
            else {
                String::from("No name")
            };
            let cpu_usage = if let Some(core_count) = sysinfo::System::physical_core_count() {
                process.cpu_usage() / core_count as f32 // normalizing process cpu usage by the number of cores
            }
            else {
                process.cpu_usage()
            };
            let memory_usage = process.memory();
            let start_time = process.start_time();
            let run_time = process.run_time();
            let accumulated_cpu_time = process.accumulated_cpu_time();
            let status = process.status().to_string();
            let item = ProcessListItem::new(
                pid.as_u32(),
                name, cpu_usage,
                memory_usage,
                start_time,
                run_time,
                accumulated_cpu_time,
                status
            );

            processes.push(item);
        }

        processes
    }

    pub fn terminate_process(&mut self, pid: u32) -> Result<bool> {
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            process.kill();
        }
        Ok(true)
    }

    /*
    pub fn get_memory_info(&self) -> MemoryItem {
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let free_memory = self.system.free_memory();
        let available_memory = self.system.available_memory();
        let memory_info = MemoryItem::new(total_memory, used_memory, free_memory, available_memory);
        memory_info
    }
*/

    //pub fn get_network_info(&self) -> NetworkItem {

    //}
}