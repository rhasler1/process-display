use std::cmp::Ordering;
use crate::models::process_list::process_item::ProcessItem;

pub enum ProcessSortOrder {
    PidInc,
    PidDec,
    NameInc,
    NameDec,
    CpuUsageInc,
    CpuUsageDec,
    MemoryUsageInc,
    MemoryUsageDec,
}

impl ProcessSortOrder {
    pub fn compare(&self, a: &ProcessItem, b: &ProcessItem) -> Ordering {
        match self {
            ProcessSortOrder::PidInc => a.pid().cmp(&b.pid()),
            ProcessSortOrder::PidDec => b.pid().cmp(&a.pid()),
            ProcessSortOrder::NameInc => a.name().cmp(&b.name()),
            ProcessSortOrder::NameDec => b.name().cmp(&a.name()),
            ProcessSortOrder::CpuUsageInc => a.cpu_usage().partial_cmp(&b.cpu_usage()).unwrap_or(Ordering::Equal),
            ProcessSortOrder::CpuUsageDec => b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(Ordering::Equal),
            ProcessSortOrder::MemoryUsageInc => a.memory_usage().cmp(&b.memory_usage()),
            ProcessSortOrder::MemoryUsageDec => b.memory_usage().cmp(&a.memory_usage()),
        }
    }
}