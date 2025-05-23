use crate::process_list::ListSortOrder;
use crate::process_list_item::ProcessListItem;
use crate::list_items_iter::ListItemsIterator;

#[derive(Default, Clone)]
pub struct ProcessListItems {
    pub list_items: Vec<ProcessListItem>,
}

impl ProcessListItems {
    pub fn new(list: Vec<ProcessListItem>) -> Self {
        Self {
            list_items: list,
        }
    }

    pub fn filter(&self, filter_text: &str) -> Self {
        Self {
            list_items: self.create_filtered_items(filter_text)
        }
    }

    fn create_filtered_items(&self, filter_text: &str) -> Vec<ProcessListItem> {
        self.list_items
            .iter()
            .filter(|item| {
                item.name().contains(filter_text) ||
                item.pid().to_string().contains(filter_text)
            })
            .cloned()
            .collect()
    }

    pub fn update_items(&mut self, new_list: Vec<ProcessListItem>) {
        self.list_items = new_list;
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

    pub fn get_item_ref(&self, idx: usize) -> Option<&ProcessListItem> {
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

    pub fn items_len(&self) -> usize {
        self.list_items.len()
    }

    pub const fn iterate(&self, start: usize, max_amount: usize) -> ListItemsIterator<'_> {
        ListItemsIterator::new(self, start, max_amount)
    }
}

#[cfg(test)]
mod test {
    use std::vec;
    use crate::process_list::ListSortOrder;
    use crate::process_list_item::ProcessListItem;
    use crate::process_list_items::ProcessListItems;

    #[test]
    fn test_default() {
        let instance = ProcessListItems::default();
        assert_eq!(instance.items_len(), 0);
        assert_eq!(instance.get_idx(4), None);
        assert_eq!(instance.get_item_ref(0), None);
    }

    #[test]
    fn test_new() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"));
        let clone_0 = item_0.clone();
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"));
        let clone_1 = item_1.clone();
        let items = vec![item_0, item_1];
        let instance = ProcessListItems::new(items);

        assert_eq!(instance.items_len(), 2);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), None);

        assert_eq!(instance.get_item_ref(0), Some(&clone_0));
        assert_eq!(instance.get_item_ref(1), Some(&clone_1));
        assert_eq!(instance.get_item_ref(2), None);
    }

    #[test]
    fn test_filter() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"));
        let clone_0 = item_0.clone();
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"));
        let _clone_1 = item_1.clone();
        let items = vec![item_0, item_1];
        let instance = ProcessListItems::new(items);

        let filtered_instance = instance.filter(&String::from("a"));
        assert_eq!(filtered_instance.items_len(), 1);
        assert_eq!(filtered_instance.get_item_ref(0), Some(&clone_0));
        assert_eq!(filtered_instance.get_item_ref(1), None);
        assert_eq!(filtered_instance.get_idx(1), Some(0));
        assert_eq!(filtered_instance.get_idx(2), None);
    }

    #[test]
    fn test_update_items() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessListItems::new(items);

        // Note: ProcessListItem's are compared by Pid.
        let item_2 = ProcessListItem::new(1, String::from("a"), 7.0, 1337, 0, 10, 10, String::from("test"));
        let item_3 = ProcessListItem::new(3, String::from("c"), 3.0, 3, 0, 10, 10, String::from("test"));
        let new_items = vec![item_2, item_3];

        let _ = instance.sort_items(&ListSortOrder::CpuUsageInc);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        let _ = instance.update_items(new_items);
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
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"));
        let item_3 = ProcessListItem::new(3, String::from("c"), 3.0, 3, 0, 10, 10, String::from("test"));
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
}
