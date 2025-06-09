use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use process_list::{ListSortOrder, MoveSelection};
use super::config::KeyConfig;
pub mod sysinfo_wrapper;
pub mod filter;
pub mod help;
pub mod error;
pub mod process;
pub mod utils;
pub mod command;
pub mod cpu;
pub mod memory;
pub mod temp;

pub trait DrawableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()>;
}

pub trait Component {
    fn event(&mut self, key: KeyEvent) -> Result<EventState>;
}

#[derive(PartialEq)]
pub enum EventState {
    Consumed,
    NotConsumed,
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
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
