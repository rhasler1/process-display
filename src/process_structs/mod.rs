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