use sysinfo::{Pid, System, Components};
use crate::models::process_list::process_item::ProcessItem;
use crate::models::items::{memory_item::MemoryItem, temp_item::TempItem, cpu_item::CpuItem};
use crate::config::Config;

// See here for refreshing system: https://crates.io/crates/sysinfo#:~:text=use%20sysinfo%3A%3ASystem,(sysinfo%3A%3AMINIMUM_CPU_UPDATE_INTERVAL)%3B%0A%7D
// note: sysinfo::MINIMUM_CPU_UPDATE_INTERVAL = 200 ms
pub struct SysInfoWrapper {
    system: System,
    components: Components,
    pub _config: Config
}

impl SysInfoWrapper {
    pub fn new(config: Config) -> Self  {
        Self {
            system: System::new_all(),
            components: Components::new_with_refreshed_list(),
            _config: config
        }
    }

    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
        self.components.refresh(false);
    }

    pub fn get_cpus(&self) -> Vec<CpuItem> {
        let mut cpus: Vec<CpuItem> = Vec::new();

        cpus.push(CpuItem::new(                // dummy item for global cpu usage
            0,
            self.system.global_cpu_usage(),
            0,
        ));

        for (id, cpu) in self.system.cpus().iter().enumerate() {
            let cpu_item = CpuItem::new(
                id + 1,                         // id=0 reserved for global cpu usage                   
                cpu.cpu_usage(),
                cpu.frequency(),
            );

            cpus.push(cpu_item);
        }

        cpus
    }

    pub fn get_processes(&self, processes: &mut Vec<ProcessItem>) {
        processes.clear();

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

            let path = if let Some(path) = process.exe() {
                if let Some(path) = path.to_str() {
                    path.to_string()
                }
                else {
                    String::from("Non-valid Unicode")
                }
            }
            else {
                String::from("Permission Denied")
            };

            let item = ProcessItem::new(
                pid.as_u32(),
                name,
                cpu_usage,
                memory_usage,
                start_time,
                run_time,
                accumulated_cpu_time,
                status,
                path,
            );

            processes.push(item);
        }
    }

    pub fn get_memory(&self, memory: &mut MemoryItem) {
        let total_memory = self.system.total_memory();          // total memory is size of RAM in bytes
        let used_memory = self.system.used_memory();            // used memory is allocated memory
        let total_swap = self.system.total_swap();
        let used_swap = self.system.used_swap();

        memory.update(total_memory, used_memory, total_swap, used_swap);
    }

    pub fn get_temps(&self, temps: &mut Vec<TempItem>) {
        temps.clear();

        for component in &self.components {
            let temp = if let Some(temp) = component.temperature() {
                temp
            }
            else {
                0_f32
            };

            let max_temp = if let Some(max_temp) = component.max() {
                max_temp
            }
            else {
                0_f32
            };

            let critical_temp = if let Some(critical_temp) = component.critical() {
                critical_temp
            }
            else {
                0_f32
            };

            let label = component.label().to_string();


            let item = TempItem::new(temp, max_temp, critical_temp, label);

            temps.push(item);
        }
    }


    pub fn terminate_process(&self, pid: u32) -> bool {
        let mut res = false;

        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            res = process.kill();
        }
        
        res
    }
}