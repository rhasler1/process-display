use std::io;
use crate::components::ListSortOrder;
use super::process_list_item::ProcessListItem;
use super::list_items_iter::ListItemsIterator;

// This structure contains a vector of type ProcessListItem.
#[derive(Default, Clone)]
pub struct ProcessListItems {
    pub list_items: Vec<ProcessListItem>,
}

impl ProcessListItems {
    // This function constructs a `new` instance of ProcessListItems and initializes field
    // list_items by passing the parameter list to the instance function create_items(list).
    pub fn new(list: &Vec<ProcessListItem>) -> Self {
        Self {
            list_items: Self::create_items(list),
        }
    }

    // This function populates a new vector of type ProcessListItem by cloning each item
    // contained in the list parameter then pushing the cloned item onto the new vector.
    // The new vector is returned.
    fn create_items(list: &Vec<ProcessListItem>) -> Vec<ProcessListItem> {
        let list_len = list.len();
        let mut items = Vec::with_capacity(list_len);
        for e in list {
            let item = e.clone();
            items.push(item);
        }
        return items;
    }

    // This function constructs a new ProcessListItems instance by filtering items
    // from the current ProcessListItems instance. Item's are filtered using the
    // ProcessListItem instance function is_match(&filter_text: &str).
    pub fn filter(&self, filter_text: String) -> Self {
        Self {
            list_items: self.list_items
                .iter()
                .filter(|item| {
                    item.is_match(&filter_text)
                })
                .map(|item| {
                    let item = item.clone();
                    item
                })
                .collect::<Vec<ProcessListItem>>(),
        }
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
        // 3. if the instance list contains an entry not in the new list, then remove entry from instance list.
        self.list_items.retain(|item| new_list.contains(item));

        // The instance list might become unsorted when updating items if sorting by usage (step 2 above).
        // Sort the list if the list is being sorted by usage.
        if *sort == ListSortOrder::UsageInc || *sort == ListSortOrder::UsageDec {
            self.sort_items(sort)?;
        }  
        Ok(())
    }

    // This function determines the parameter item's insert index position in the instance list given a ListSortOrder.
    fn insert_item_idx(&mut self, item: &ProcessListItem, sort: &ListSortOrder) -> usize {
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
            ListSortOrder::UsageInc => {
                self.list_items
                    .binary_search_by(|probe| probe.cpu_usage().partial_cmp(&item.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::UsageDec => {
                self.list_items
                .binary_search_by(|probe| item.cpu_usage().partial_cmp(&probe.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal))
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
            ListSortOrder::UsageInc => {
                self.list_items.sort_by(|a, b| a.cpu_usage().partial_cmp(&b.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
            }
            ListSortOrder::UsageDec => {
                self.list_items.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
            }
        }
        Ok(())
    }

    // This function gets the reference to an item given an index into the instance list.
    pub fn get_item(&self, idx: usize) -> Option<&ProcessListItem> {
        let list_len = self.list_len();
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
    pub fn list_len(&self) -> usize {
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
    use crate::process_structs::process_list_items::ProcessListItems;
    use crate::process_structs::process_list_item::ProcessListItem;
    //use crate::components::{filter, ListSortOrder};

    #[test]
    fn test_default() {
        let instance = ProcessListItems::default();
        assert_eq!(instance.list_len(), 0);
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

        assert_eq!(instance.list_len(), 2);
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

        let filtered_instance = instance.filter(String::from("a"));
        assert_eq!(filtered_instance.list_len(), 1);
        assert_eq!(filtered_instance.get_item(0), Some(&clone_0));
        assert_eq!(filtered_instance.get_item(1), None);
        assert_eq!(filtered_instance.get_idx(1), Some(0));
        assert_eq!(filtered_instance.get_idx(2), None);
    }

    #[test]
    fn test_insert_item_idx() {
        
    }

    #[test]
    fn test_update_items() {

    }

    #[test]
    fn test_sort_items() {

    }
}