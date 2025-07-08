use crate::components::sysinfo_wrapper::SysInfoWrapper;
use super::process_list::ListSortOrder;
use super::process_list_item::ProcessListItem;
use super::list_items_iter::ListItemsIterator;

#[derive(Default, Clone)]
pub struct ProcessListItems {
    pub list_items: Vec<ProcessListItem>,
}

impl ProcessListItems {
    pub fn new(sysinfo: &SysInfoWrapper) -> Self {
        let mut processes = Vec::new();

        sysinfo.get_processes(&mut processes);
        
        Self {
            list_items: processes,
        }
    }

    pub fn filter(&self, filter_text: &str) -> Self {
        let list_items = self.list_items
            .iter()
            .filter(|item| {
                item.name().contains(filter_text) ||
                item.pid().to_string().contains(filter_text)
            })
            .cloned()
            .collect();

        Self {
            list_items
        }
    }

    pub fn update(&mut self, sysinfo: &SysInfoWrapper, filter_text: &str) {
        sysinfo.get_processes(&mut self.list_items);

        if !filter_text.is_empty() {
            self.list_items.retain(|item| {
                item.name().contains(filter_text) ||
                item.pid().to_string().contains(filter_text)
            });
        }
    }

    pub fn sort_items(&mut self, sort: &ListSortOrder) {
        match sort {
            ListSortOrder::PidInc => {
                self.list_items.sort_by_key(|a| a.pid());
            }
            ListSortOrder::PidDec => {
                self.list_items.sort_by_key(|b| std::cmp::Reverse (b.pid()));
            }
            ListSortOrder::NameInc => {
                self.list_items.sort_by_key(|a| a.name().to_string());
            }
            ListSortOrder::NameDec => {
                self.list_items.sort_by_key(|b| std::cmp::Reverse (b.name().to_string()));
            }
            ListSortOrder::CpuUsageInc => {
                self.list_items.sort_by(|a, b| a.cpu_usage().partial_cmp(&b.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
            }
            ListSortOrder::CpuUsageDec => {
                self.list_items.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
            }
            ListSortOrder::MemoryUsageInc => {
                self.list_items.sort_by_key(|a| a.memory_usage());
            }
            ListSortOrder::MemoryUsageDec => {
                self.list_items.sort_by_key(|b| std::cmp::Reverse (b.memory_usage()));
            }
        }
    }

    pub fn get_item(&self, idx: usize) -> Option<&ProcessListItem> {
        self.list_items.get(idx)
    }

    pub fn get_idx(&self, pid: u32) -> Option<usize> {
        if let Some(idx) = self.list_items
            .iter()
            .position(|item| item.pid() == pid)
        {
            return Some(idx);
        }
        None
    }

    pub fn len(&self) -> usize {
        self.list_items.len()
    }

    pub const fn iterate(&self, start: usize, max_amount: usize) -> ListItemsIterator<'_> {
        ListItemsIterator::new(self, start, max_amount)
    }
}


/*
// TODO: come up with new unit testing strategy
#[cfg(test)]
mod test {
    use std::vec;
    use crate::components::sysinfo_wrapper::{self, SysInfoWrapper};
    use crate::config::{self, Config};
    use crate::models::process_list::ListSortOrder;
    use crate::models::process_list_item::ProcessListItem;
    use crate::models::process_list_items::ProcessListItems;

    #[test]
    fn test_default() {
        let instance = ProcessListItems::default();
        assert_eq!(instance.len(), 0);
        assert_eq!(instance.get_idx(4), None);
        assert_eq!(instance.get_item(0), None);
    }

    #[test]
    fn test_new() {
        let config = Config::default();
        let sysinfo_wrapper = SysInfoWrapper::new(config);
        sysinfo_wrapper.refresh_all();

        /*let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let clone_0 = item_0.clone();
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let clone_1 = item_1.clone();
        let items = vec![item_0, item_1];
        let instance = ProcessListItems::new(items);*/

        let pl_instance = ProcessListItems::new(&sysinfo_wrapper);

        assert_eq!(pl_instance.len(), 2);
        assert_eq!(pl_instance.get_idx(1), Some(0));
        assert_eq!(pl_instance.get_idx(2), Some(1));
        assert_eq!(pl_instance.get_idx(3), None);

        assert_eq!(pl_instance.get_item(0), Some(&clone_0));
        assert_eq!(pl_instance.get_item(1), Some(&clone_1));
        assert_eq!(pl_instance.get_item(2), None);
    }

    #[test]
    fn test_filter() {
        let config = Config::new 
        let system_wrapper = SysInfoWrapper::new(config)

        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let clone_0 = item_0.clone();
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let _clone_1 = item_1.clone();
        let items = vec![item_0, item_1];
        let instance = ProcessListItems::new(items);

        let filtered_instance = instance.filter(&String::from("a"));
        assert_eq!(filtered_instance.len(), 1);
        assert_eq!(filtered_instance.get_item(0), Some(&clone_0));
        assert_eq!(filtered_instance.get_item(1), None);
        assert_eq!(filtered_instance.get_idx(1), Some(0));
        assert_eq!(filtered_instance.get_idx(2), None);
    }

    #[test]
    fn test_update_items() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessListItems::new(items);

        // Note: ProcessListItem's are compared by Pid.
        let item_2 = ProcessListItem::new(1, String::from("a"), 7.0, 1337, 0, 10, 10, String::from("test"), String::from("test"));
        let item_3 = ProcessListItem::new(3, String::from("c"), 3.0, 3, 0, 10, 10, String::from("test"), String::from("test"));
        let new_items = vec![item_2, item_3];

        let _ = instance.sort_items(&ListSortOrder::CpuUsageInc);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        let _ = instance.update(new_items);
        let _ = instance.sort_items(&ListSortOrder::CpuUsageInc);
        // Pid 2 is not in new_items so it should be removed from the instance list.
        assert_eq!(instance.get_idx(2), None);
        // Pid 3 cpu usage is 3.0 so it should be first in the instance list.
        assert_eq!(instance.get_idx(3), Some(0));
        // Pid 1 cpu usage is updated to 7.0 so it should be last in the instance list.
        assert_eq!(instance.get_idx(1), Some(1));
    }

    #[test]
    fn test_sort_items() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let item_3 = ProcessListItem::new(3, String::from("c"), 3.0, 3, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1, item_3];
        let mut instance = ProcessListItems::new(items);

        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(2));
        let _ = instance.sort_items(&ListSortOrder::CpuUsageInc);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(2));

        let _ = instance.sort_items(&ListSortOrder::CpuUsageDec);
        assert_eq!(instance.get_idx(1), Some(2));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(0));

        let _ = instance.sort_items(&ListSortOrder::NameInc);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(2));

        let _ = instance.sort_items(&ListSortOrder::NameDec);
        assert_eq!(instance.get_idx(1), Some(2));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(0));

        let _ = instance.sort_items(&ListSortOrder::PidInc);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(2));

        let _ = instance.sort_items(&ListSortOrder::PidDec);
        assert_eq!(instance.get_idx(1), Some(2));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(0));

        let _ = instance.sort_items(&ListSortOrder::MemoryUsageInc);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(2));

        let _ = instance.sort_items(&ListSortOrder::MemoryUsageDec);
        assert_eq!(instance.get_idx(1), Some(2));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), Some(0));
    }
}*/