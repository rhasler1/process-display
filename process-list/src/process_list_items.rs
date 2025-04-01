use std::io;
use crate::process_list::ListSortOrder;
use crate::process_list_item::ProcessListItem;
use crate::list_items_iter::ListItemsIterator;

// This structure contains a vector of type ProcessListItem.
#[derive(Default, Clone)]
pub struct ProcessListItems {
    pub list_items: Vec<ProcessListItem>,
}

impl ProcessListItems {
    // creator
    pub fn new(list: &Vec<ProcessListItem>) -> Self {
        Self {
            list_items: Self::create_items(list),
        }
    }

    // creator helper
    fn create_items(list: &Vec<ProcessListItem>) -> Vec<ProcessListItem> {
        list.iter().cloned().collect()
    }

    // creator for filtered list, call on existing list
    pub fn filter(&self, filter_text: &String) -> Self {
        Self {
            list_items: self.create_filtered_items(filter_text)
        }
    }

    fn create_filtered_items(&self, filter_text: &String) -> Vec<ProcessListItem> {
        self.list_items
            .iter()
            .filter(|item| {item.is_match(filter_text)})
            .cloned()
            .collect()
    }

    // This function updates the ProcessListItems instance field list_items by adding new items, updating fields of
    // existing items, and removing old items given the parameters new_list and sort. Item's can be sorted by a
    // ListSortOrder, the sort of a list determines the items insert position.
    pub fn update_items(&mut self, new_list: &Vec<ProcessListItem>, sort: &ListSortOrder) -> io::Result<()> {
        for e in new_list {
             // 1. If the new list contains an entry not in the instance list, then add entry to instance list.
            if !self.list_items.contains(e) {
                // 1.1 Get the index to insert the item `e`.
                let idx = self.insert_item_idx(e, sort);
                // 1.2 Clone the item from the new list and insert in instance list.
                let item = e.clone();
                self.list_items.insert(idx, item);
            }
            // 2. If the new list contains updated information for a process in the instance list, then update the instance list item.
            else if let Some(instance_item) =
                self.list_items.iter_mut().find(|item| item == &e) { *instance_item = e.clone(); }
        }
        // 3. If the instance list contains an entry not in the new list, then remove entry from instance list.
        self.list_items.retain(|item| new_list.contains(item));

        // The instance list might become unsorted when updating items if sorting by usage (step 2 above).
        // Sort the list if the list is being sorted by usage.
        if *sort == ListSortOrder::CpuUsageInc || *sort == ListSortOrder::CpuUsageDec || *sort == ListSortOrder::MemoryUsageInc || *sort == ListSortOrder::MemoryUsageDec {
            self.sort_items(sort)?;
        }  
        Ok(())
    }

    // This function determines the parameter item's insert index position in the instance list given a ListSortOrder.
    fn insert_item_idx(&self, item: &ProcessListItem, sort: &ListSortOrder) -> usize {
        match sort {
            ListSortOrder::PidInc => {
                self.list_items
                    .binary_search_by(|probe| probe.pid().cmp(&item.pid()))
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::PidDec => {
                self.list_items
                    .binary_search_by(|probe| probe.pid().cmp(&item.pid()).reverse())
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::NameInc => {
                self.list_items
                    .binary_search_by(|probe| probe.name().cmp(&item.name()))
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::NameDec => {
                self.list_items
                    .binary_search_by(|probe| probe.name().cmp(&item.name()).reverse())
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::CpuUsageInc => {
                self.list_items
                    .binary_search_by(|probe| probe.cpu_usage().partial_cmp(&item.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::CpuUsageDec => {
                self.list_items
                    .binary_search_by(|probe| item.cpu_usage().partial_cmp(&probe.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::MemoryUsageInc => {
                self.list_items
                    .binary_search_by(|probe| probe.memory_usage().cmp(&item.memory_usage()))
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::MemoryUsageDec => {
                self.list_items
                    .binary_search_by(|probe| item.memory_usage().cmp(&probe.memory_usage()))
                    .unwrap_or_else(|index| index)
            }
        }
    }

    // This function sorts the instance list by parameter sort.
    pub fn sort_items(&mut self, sort: &ListSortOrder) -> io::Result<()> {
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
        Ok(())
    }

    // GETTERS
    // This function gets the reference to an item given an index into the instance list.
    pub fn get_item(&self, idx: usize) -> Option<&ProcessListItem> {
        let list_len = self.items_len();
        let max_idx = list_len.saturating_sub(1);
        if list_len == 0 || max_idx < idx {
            return None
        }
        let item = self.list_items.get(idx);
        item
    }

    // This function gets the index of an item in the instance list given the item's pid.
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

    // This function gets the length of the instance list.
    pub fn items_len(&self) -> usize {
        self.list_items.len()
    }

    // This function returns a ListItemIterator instance given a start position and max number of iterations. 
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
    fn test_insert_item_idx() {
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessListItems::new(&items);
        let item_to_insert = ProcessListItem::new(3, String::from("c"), 1.5, 0);

        // The item's must be sorted by the argument provided to insert_item_idx(sort) to produce accurate results.
        let _ = instance.sort_items(&ListSortOrder::NameDec);
        let idx = instance.insert_item_idx(&item_to_insert, &ListSortOrder::NameDec);
        assert_eq!(idx, 0);
        let _ = instance.sort_items(&ListSortOrder::NameInc);
        let idx = instance.insert_item_idx(&item_to_insert, &ListSortOrder::NameInc);
        assert_eq!(idx, 2);
        let _ = instance.sort_items(&ListSortOrder::PidDec);
        let idx = instance.insert_item_idx(&item_to_insert, &ListSortOrder::PidDec);
        assert_eq!(idx, 0);
        let _ = instance.sort_items(&ListSortOrder::PidInc);
        let idx = instance.insert_item_idx(&item_to_insert, &ListSortOrder::PidInc);
        assert_eq!(idx, 2);
        let _ = instance.sort_items(&ListSortOrder::CpuUsageDec);
        let idx = instance.insert_item_idx(&item_to_insert, &ListSortOrder::CpuUsageDec);
        assert_eq!(idx, 1);
        let _ = instance.sort_items(&ListSortOrder::CpuUsageInc);
        let idx = instance.insert_item_idx(&item_to_insert, &ListSortOrder::CpuUsageInc);
        assert_eq!(idx, 1);
        let _ = instance.sort_items(&ListSortOrder::MemoryUsageDec);
        let idx = instance.insert_item_idx(&item_to_insert, &ListSortOrder::MemoryUsageDec);
        assert_eq!(idx, 2);
        let _ = instance.sort_items(&ListSortOrder::MemoryUsageInc);
        let idx = instance.insert_item_idx(&item_to_insert, &ListSortOrder::MemoryUsageInc);
        assert_eq!(idx, 0)
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
        let _ = instance.update_items(&new_items, &ListSortOrder::CpuUsageInc);
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