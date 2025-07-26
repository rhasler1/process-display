use sysinfo::{Components, Networks, Pid, System};
use crate::models::items::network_item::NetworkItem;
use crate::models::items::{memory_item::MemoryItem, temp_item::TempItem, cpu_item::CpuItem, process_item::ProcessItem};
use crate::config::Config;
use crate::services::{ItemProvider, VecProvider};

// See here for refreshing system: https://crates.io/crates/sysinfo#:~:text=use%20sysinfo%3A%3ASystem,(sysinfo%3A%3AMINIMUM_CPU_UPDATE_INTERVAL)%3B%0A%7D
// note: sysinfo::MINIMUM_CPU_UPDATE_INTERVAL = 200 ms
pub struct SysInfoService {
    system: System,
    components: Components,
    networks: Networks,
    pub _config: Config
}

impl SysInfoService {
    pub fn new(config: Config) -> Self  {
        Self {
            system: System::new_all(),
            components: Components::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            _config: config
        }
    }

    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
        self.components.refresh(false);
        self.networks.refresh(true);
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


    pub fn terminate_process(&self, pid: u32) -> bool {
        let mut res = false;

        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            res = process.kill();
        }
        
        res
    }
}

impl ItemProvider<NetworkItem> for SysInfoService {
    fn fetch_item(&self) -> NetworkItem {
        let mut tx = 0;
        let mut rx = 0;
        let mut total_tx = 0;
        let mut total_rx = 0;

        for (_interface_name, network) in &self.networks {
            tx += network.transmitted();
            rx += network.received();
            total_tx += network.total_transmitted();
            total_rx += network.total_received();
        }

        NetworkItem::new(tx, rx, total_tx, total_rx)
    }
}

impl ItemProvider<MemoryItem> for SysInfoService {
    fn fetch_item(&self) -> MemoryItem {
        let total_memory = self.system.total_memory();          // total memory is size of RAM in bytes
        let used_memory = self.system.used_memory();            // used memory is allocated memory
        let total_swap = self.system.total_swap();
        let used_swap = self.system.used_swap();

        MemoryItem::new(total_memory, used_memory, total_swap, used_swap)
    }
}

impl VecProvider<TempItem> for SysInfoService {
    fn fetch_items(&self) -> Vec<TempItem> {
        let mut temps: Vec<TempItem> = Vec::new();

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

        temps
    }
}

impl VecProvider<ProcessItem> for SysInfoService {
    fn fetch_items(&self) -> Vec<ProcessItem> {
        let mut processes: Vec<ProcessItem> = Vec::new();

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

        return processes;
    }
}