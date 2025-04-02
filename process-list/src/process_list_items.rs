use std::collections::HashMap;
use crate::process_list::ListSortOrder;
use crate::process_list_item::ProcessListItem;
use crate::list_items_iter::ListItemsIterator;

#[derive(Default, Clone)]
pub struct ProcessListItems {
    pub list_items: Vec<ProcessListItem>,
}

impl ProcessListItems {
    // constructor
    pub fn new(list: &Vec<ProcessListItem>) -> Self {
        Self {
            list_items: Self::create_items(list),
        }
    }

    // constructor helper
    fn create_items(list: &Vec<ProcessListItem>) -> Vec<ProcessListItem> {
        list.iter().cloned().collect()
    }

    // constructor for filtered list
    pub fn filter(&self, filter_text: &String) -> Self {
        Self {
            list_items: self.create_filtered_items(filter_text)
        }
    }

    // filtered constructor helper 
    fn create_filtered_items(&self, filter_text: &String) -> Vec<ProcessListItem> {
        self.list_items
            .iter()
            .filter(|item| {item.is_match(filter_text)})
            .cloned()
            .collect()
    }

    // updates existing items, removes old items, and adds new items
    // sort order is ruined by this function, call sort after use
    pub fn update_items(&mut self, new_list: &Vec<ProcessListItem>) {
        let mut updated_list: Vec<ProcessListItem> = Vec::new();
        let new_map: HashMap<_,_> = new_list.iter().map(|item| (item.pid(),item)).collect();

        for item in &mut self.list_items {
            if let Some(&updated_item) = new_map.get(&item.pid()) {
                *item = updated_item.clone();
                updated_list.push(item.clone());
            }
        }

        for new_item in new_list {
            if !updated_list.contains(new_item) {
                updated_list.push(new_item.clone())
            }
        }

        self.list_items = updated_list;
    }

    // sort by list sort order
    pub fn sort_items(&mut self, sort: &ListSortOrder) {
        match sort {
            ListSortOrder::PidInc => {
                self.list_items.sort_by(|a, b| a.pid().cmp(&b.pid()));
            }
            ListSortOrder::PidDec => {
                self.list_items.sort_by(|a, b| b.pid().cmp(&a.pid()));
            }
            ListSortOrder::NameInc => {
                self.list_items.sort_by(|a, b| a.name().cmp(&b.name()));
            }
            ListSortOrder::NameDec => {
                self.list_items.sort_by(|a, b| b.name().cmp(&a.name()));
            }
            ListSortOrder::CpuUsageInc => {
                self.list_items.sort_by(|a, b| a.cpu_usage().partial_cmp(&b.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
            }
            ListSortOrder::CpuUsageDec => {
                self.list_items.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
            }
            ListSortOrder::MemoryUsageInc => {
                self.list_items.sort_by(|a, b| a.memory_usage().cmp(&b.memory_usage()));
            }
            ListSortOrder::MemoryUsageDec => {
                self.list_items.sort_by(|a, b| b.memory_usage().cmp(&a.memory_usage()));
            }
        }
    }

    // GETTERS
    // gets the reference to an item given an index
    pub fn get_item(&self, idx: usize) -> Option<&ProcessListItem> {
        let list_len = self.items_len();
        let max_idx = list_len.saturating_sub(1);
        if list_len == 0 || max_idx < idx {
            return None
        }
        let item = self.list_items.get(idx);
        item
    }

    // gets the index of an item given the item's pid.
    pub fn get_idx(&self, pid: u32) -> Option<usize> {
        if let Some(idx) = self.list_items
            .iter()
            .position(|item| item.pid() == pid)
        {
            return Some(idx);
        }
        else {
            return None;
        }
    }

    // gets the length of the instance list.
    pub fn items_len(&self) -> usize {
        self.list_items.len()
    }

    // returns a ListItemIterator instance given a start position and max number of iterations. 
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
        assert_eq!(instance.get_item(0), None);
    }

    #[test]
    fn test_new() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let clone_0 = item_0.clone();
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let clone_1 = item_1.clone();
        let items = vec![item_0, item_1];
        let instance = ProcessListItems::new(&items);

        assert_eq!(instance.items_len(), 2);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        assert_eq!(instance.get_idx(3), None);

        assert_eq!(instance.get_item(0), Some(&clone_0));
        assert_eq!(instance.get_item(1), Some(&clone_1));
        assert_eq!(instance.get_item(2), None);
    }

    #[test]
    fn test_filter() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let clone_0 = item_0.clone();
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let _clone_1 = item_1.clone();
        let items = vec![item_0, item_1];
        let instance = ProcessListItems::new(&items);

        let filtered_instance = instance.filter(&String::from("a"));
        assert_eq!(filtered_instance.items_len(), 1);
        assert_eq!(filtered_instance.get_item(0), Some(&clone_0));
        assert_eq!(filtered_instance.get_item(1), None);
        assert_eq!(filtered_instance.get_idx(1), Some(0));
        assert_eq!(filtered_instance.get_idx(2), None);
    }

    #[test]
    fn test_update_items() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessListItems::new(&items);

        // Note: ProcessListItem's are compared by Pid.
        let item_2 = ProcessListItem::new(1, String::from("a"), 7.0, 1337);
        let item_3 = ProcessListItem::new(3, String::from("c"), 3.0, 3);
        let new_items = vec![item_2, item_3];

        let _ = instance.sort_items(&ListSortOrder::CpuUsageInc);
        assert_eq!(instance.get_idx(1), Some(0));
        assert_eq!(instance.get_idx(2), Some(1));
        let _ = instance.update_items(&new_items);
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
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let item_3 = ProcessListItem::new(3, String::from("c"), 3.0, 3);
        let items = vec![item_0, item_1, item_3];
        let mut instance = ProcessListItems::new(&items);

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