pub mod process_list;
pub mod process_item;
pub mod process_item_iter;

// possible set of values (known at compile time)
// Variants correspond to static comparator functions
// found at ../items/process_list_item.rs
#[derive(PartialEq, Clone)]
pub enum ProcessItemSortOrder {
    PidInc,
    PidDec,
    NameInc,
    NameDec,
    CpuUsageInc,
    CpuUsageDec,
    MemoryUsageInc,
    MemoryUsageDec,
}

// This function maps user's dynamic key press to static sorting strategy.
// Supported mappings are explicitly stated in function match statement.
// Returns Some(ProcessItemSortOrder::Variant) or None.
use crossterm::event::KeyEvent;
use crate::config::KeyConfig;
pub fn map_key_to_process_sort(key: KeyEvent, key_config: &KeyConfig) -> Option<ProcessItemSortOrder> {
    match key.code {
        code if code == key_config.sort_cpu_usage_dec => Some(ProcessItemSortOrder::CpuUsageDec),
        code if code == key_config.sort_cpu_usage_inc => Some(ProcessItemSortOrder::CpuUsageInc),
        code if code == key_config.sort_memory_usage_dec => Some(ProcessItemSortOrder::MemoryUsageDec),
        code if code == key_config.sort_memory_usage_inc => Some(ProcessItemSortOrder::MemoryUsageInc),
        code if code == key_config.sort_pid_dec => Some(ProcessItemSortOrder::PidDec),
        code if code == key_config.sort_pid_inc => Some(ProcessItemSortOrder::PidInc),
        code if code == key_config.sort_name_dec => Some(ProcessItemSortOrder::NameDec),
        code if code == key_config.sort_name_inc => Some(ProcessItemSortOrder::NameInc),
        _ => None,
    }
}