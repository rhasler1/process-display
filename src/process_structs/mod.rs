use crossterm::event::KeyEvent;
use super::config::KeyConfig;
use super::process_structs::process_list::MoveSelection;

pub mod list_items_iter;
pub mod list_iter;
pub mod process_list_item;
pub mod process_list_items;
pub mod process_list;

#[derive(PartialEq, Clone)]
pub enum ListSortOrder {
    PidInc,
    PidDec,
    NameInc,
    NameDec,
    CpuUsageInc,
    CpuUsageDec,
    MemoryUsageInc,
    MemoryUsageDec,
}

impl Default for ListSortOrder {
    fn default() -> Self {
        ListSortOrder::CpuUsageDec
    }
}

pub fn common_nav(key: KeyEvent, key_config: &KeyConfig) -> Option<MoveSelection> {
    if key.code == key_config.move_down {
        Some(MoveSelection::Down)
    }
    else if key.code == key_config.move_bottom {
        Some(MoveSelection::End)
    }
    else if key.code == key_config.move_up {
        Some(MoveSelection::Up)
    }
    else if key.code == key_config.move_top {
        Some(MoveSelection::Top)
    }
    else {
        None
    }
}

pub fn common_sort(key: KeyEvent, key_config: &KeyConfig) -> Option<ListSortOrder> {
    if key.code == key_config.sort_cpu_usage_dec {
        Some(ListSortOrder::CpuUsageDec)
    }
    else if key.code == key_config.sort_cpu_usage_inc {
        Some(ListSortOrder::CpuUsageInc)
    }
    else if key.code == key_config.sort_memory_usage_dec {
        Some(ListSortOrder::MemoryUsageDec)
    }
    else if key.code == key_config.sort_memory_usage_inc {
        Some(ListSortOrder::MemoryUsageInc)
    }
    else if key.code == key_config.sort_pid_dec {
        Some(ListSortOrder::PidDec)
    }
    else if key.code == key_config.sort_pid_inc {
        Some(ListSortOrder::PidInc)
    }
    else if key.code == key_config.sort_name_dec {
        Some(ListSortOrder::NameDec)
    }
    else if key.code == key_config.sort_name_inc {
        Some(ListSortOrder::NameInc)
    }
    else {
        None
    }
}