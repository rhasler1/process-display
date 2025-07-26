use std::ops::Div;

use crate::{models::{Filterable, Sortable}};

#[derive(Clone, Copy, PartialEq)]
pub enum ProcessItemSortOrder {
    PidInc,
    PidDec,
    NameInc,
    NameDec,
    CpuUsageInc,
    CpuUsageDec,
    MemoryUsageInc,
    MemoryUsageDec,
    StatusInc,
    StatusDec,
    RuntimeInc,
    RuntimeDec,
}

#[derive(Default, Clone)]
pub struct ProcessItem {
    pid:                    u32,
    name:                   String,
    cpu_usage:              f32,
    memory_usage:           u64,
    //read_bytes:             u64,
    //written_bytes:          u64,
    //total_read_bytes:       u64,
    //total_written_bytes:    u64,
    start_time:             u64,
    run_time:               u64,
    accumulated_cpu_time:   u64,
    status:                 String,
    path:                   String,
}

impl ProcessItem {
    pub fn new(
        pid:                    u32,
        name:                   String,
        cpu_usage:              f32,
        memory_usage:           u64,
        //read_bytes:             u64,
        //written_bytes:          u64,
        //total_read_bytes:       u64,
        //total_written_bytes:    u64,
        start_time:             u64,
        run_time:               u64,
        accumulated_cpu_time:   u64,
        status:                 String,
        path:                   String,
    ) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
            memory_usage,
            //read_bytes,
            //written_bytes,
            //total_read_bytes,
            //total_written_bytes,
            start_time,
            run_time,
            accumulated_cpu_time,
            status,
            path,
        }
    }

    // GETTERS
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub fn memory_usage(&self) -> u64 {
        self.memory_usage
    }

    /*pub fn read_bytes(&self) -> u64 {
        self.read_bytes
    }

    pub fn written_bytes(&self) -> u64 {
        self.written_bytes
    }

    pub fn total_read_bytes(&self) -> u64 {
        self.total_read_bytes
    }

    pub fn total_written_bytes(&self) -> u64 {
        self.total_written_bytes
    }*/

    pub fn start_time(&self) -> u64 {
        self.start_time
    }

    pub fn run_time(&self) -> u64 {
        self.run_time
    }

    pub fn run_time_dd_hh_mm_ss(&self) -> String {
        let time_in_s = self.run_time;

        let ss =  time_in_s % 60;
        let mm = (time_in_s / 60) % 60;
        let hh = (time_in_s / 3600) % 24;
        let dd = hh / 86400;

        format!("{:0>2}D {:0>2}H {:0>2}M {:0>2}S", dd, hh, mm, ss)
    }

    pub fn accumulated_cpu_time(&self) -> u64 {
        self.accumulated_cpu_time
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

// PartialEq is needed for comparison, e.g., calling contains
impl PartialEq for ProcessItem {
    fn eq(&self, other: &Self) -> bool {
        self.pid.eq(&other.pid)
    }
}

impl Filterable for ProcessItem {
    fn matches_filter(&self, filter: &str) -> bool {
        // by pid
        if let Some(pid_str) = filter.strip_prefix("pid>") {
            if let Ok(threshold) = pid_str.trim().parse::<u32>() {
                return self.pid() > threshold;
            }
        } else if let Some(pid_str) = filter.strip_prefix("pid<") {
            if let Ok(threshold) = pid_str.trim().parse::<u32>() {
                return self.pid() < threshold;
            }
        } else if let Some(pid_str) = filter.strip_prefix("pid=") {
            if let Ok(target) = pid_str.trim().parse::<u32>() {
                return self.pid() == target;
            }
        }

        // by cpu
        if let Some(cpu_str) = filter.strip_prefix("cpu>") {
            if let Ok(threshold) = cpu_str.trim().parse::<f32>() {
                return self.cpu_usage() > threshold;
            }
        } else if let Some(cpu_str) = filter.strip_prefix("cpu<") {
            if let Ok(threshold) = cpu_str.trim().parse::<f32>() {
                return self.cpu_usage() < threshold;
            }
        } else if let Some(cpu_str) = filter.strip_prefix("cpu=") {
            if let Ok(target) = cpu_str.trim().parse::<f32>() {
                return self.cpu_usage() == target;
            }
        }

        // by mem
        if let Some(mem_str) = filter.strip_prefix("mem>") {
            if let Ok(threshold) = mem_str.trim().parse::<u64>() {
                return (self.memory_usage().div(1000000)) > threshold;
            }
        } else if let Some(mem_str) = filter.strip_prefix("mem<") {
            if let Ok(threshold) = mem_str.trim().parse::<u64>() {
                return self.memory_usage().div(1000000) < threshold;
            }
        } else if let Some(mem_str) = filter.strip_prefix("mem=") {
            if let Ok(target) = mem_str.trim().parse::<u64>() {
                return self.memory_usage().div(1000000) == target;
            }
        }

        // by name
        self.name.to_lowercase().contains(&filter.to_lowercase()) 
    }
}

impl Sortable<ProcessItemSortOrder> for ProcessItem {
    fn cmp_with(&self, other: &Self, sort: &ProcessItemSortOrder) -> std::cmp::Ordering {
        match sort {
            ProcessItemSortOrder::PidInc =>             self.pid.cmp(&other.pid),
            ProcessItemSortOrder::PidDec =>             other.pid.cmp(&self.pid),
            ProcessItemSortOrder::NameInc =>            self.name.cmp(&other.name),
            ProcessItemSortOrder::NameDec =>            other.name.cmp(&self.name),
            ProcessItemSortOrder::CpuUsageInc =>        self.cpu_usage.partial_cmp(&other.cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
            ProcessItemSortOrder::CpuUsageDec =>        other.cpu_usage.partial_cmp(&self.cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
            ProcessItemSortOrder::MemoryUsageInc =>     self.memory_usage.cmp(&other.memory_usage),
            ProcessItemSortOrder::MemoryUsageDec =>     other.memory_usage.cmp(&self.memory_usage),
            ProcessItemSortOrder::StatusInc =>          self.status.cmp(&other.status),
            ProcessItemSortOrder::StatusDec =>          other.status.cmp(&self.status),
            ProcessItemSortOrder::RuntimeInc =>         self.run_time.cmp(&other.run_time),
            ProcessItemSortOrder::RuntimeDec =>         other.run_time.cmp(&self.run_time),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::models::Filterable;

    use super::ProcessItem;

    #[test]
    fn test_constructors() {
        let instance = ProcessItem::default();
        assert_eq!(instance.pid, 0);
        assert!(String::is_empty(&instance.name));
        assert_eq!(instance.cpu_usage, 0.0);
        assert_eq!(instance.memory_usage, 0);
        assert_eq!(instance.start_time, 0);
        assert_eq!(instance.run_time, 0);
        assert_eq!(instance.accumulated_cpu_time, 0);
        assert!(String::is_empty(&instance.status));

        let instance = ProcessItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        assert_eq!(instance.pid, 1);
        assert_eq!(instance.name, String::from("a"));
        assert_eq!(instance.cpu_usage, 1.0);
        assert_eq!(instance.memory_usage, 1);
        assert_eq!(instance.start_time, 0);
        assert_eq!(instance.run_time, 10);
        assert_eq!(instance.accumulated_cpu_time, 10);
        assert_eq!(instance.status, String::from("test"));
        
    }

    #[test]
    fn test_instance_functions() {
        let instance_0 = ProcessItem::default();
        let instance_1 = ProcessItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));

        assert_eq!(instance_0.pid(), instance_0.pid);
        assert_eq!(instance_0.name(), instance_0.name);
        assert_eq!(instance_0.cpu_usage(), instance_0.cpu_usage);
        assert_eq!(instance_0.memory_usage(), instance_0.memory_usage);
        assert_eq!(instance_0.start_time(), instance_0.start_time);
        assert_eq!(instance_0.run_time(), instance_0.run_time);
        assert_eq!(instance_0.accumulated_cpu_time(), instance_0.accumulated_cpu_time);
        assert_eq!(instance_0.status(), instance_0.status);
        assert_eq!(instance_0.matches_filter(""), true);
        assert_eq!(instance_0.matches_filter("a"), false);
        assert_eq!(instance_0.matches_filter(&format!("pid={}", &instance_0.pid())), true);

        assert_eq!(instance_1.pid(), instance_1.pid);
        assert_eq!(instance_1.name(), instance_1.name);
        assert_eq!(instance_1.cpu_usage(), instance_1.cpu_usage);
        assert_eq!(instance_1.memory_usage(), instance_1.memory_usage);
        assert_eq!(instance_0.start_time(), instance_0.start_time);
        assert_eq!(instance_0.run_time(), instance_0.run_time);
        assert_eq!(instance_0.accumulated_cpu_time(), instance_0.accumulated_cpu_time);
        assert_eq!(instance_0.status(), instance_0.status);
        assert_eq!(instance_1.matches_filter("a"), true);
        assert_eq!(instance_1.matches_filter("aa"), false);
        assert_eq!(instance_1.matches_filter(&format!("pid={}", &instance_1.pid.to_string())), true);
    }
}