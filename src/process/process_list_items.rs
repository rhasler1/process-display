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
                    .binary_search_by(|probe| {
                        if probe.is_cpu() && item.is_cpu() {
                            probe.cpu_usage().partial_cmp(&item.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        else {
                            probe.memory_usage().partial_cmp(&item.memory_usage()).unwrap_or(std::cmp::Ordering::Equal)
                        }
                    })
                    .unwrap_or_else(|index| index)
            }
            ListSortOrder::UsageDec => {
                self.list_items
                .binary_search_by(|probe| {
                    if probe.is_cpu() && item.is_cpu() {
                        item.cpu_usage().partial_cmp(&probe.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    else {
                        item.memory_usage().partial_cmp(&probe.memory_usage()).unwrap_or(std::cmp::Ordering::Equal)
                    }
                })
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
                self.list_items
                    .sort_by(
                        |a, b| a.name().cmp(&b.name())
                    );
            }
            ListSortOrder::NameDec => {
                self.list_items.sort_by(|a, b| b.name().cmp(&a.name()));
            }
            // TODO: remove all occurrences of .unwrap()
            ListSortOrder::UsageInc => {
                self.list_items.sort_by(|a, b| {
                    if a.is_cpu() && b.is_cpu() {
                        a.cpu_usage().unwrap().partial_cmp(&b.cpu_usage().unwrap()).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    else {
                        a.memory_usage().unwrap().partial_cmp(&b.memory_usage().unwrap()).unwrap_or(std::cmp::Ordering::Equal)
                    }
                })
            }
            ListSortOrder::UsageDec => {
                self.list_items.sort_by(|a, b| {
                    if a.is_cpu() && b.is_cpu() {
                        b.cpu_usage().unwrap().partial_cmp(&a.cpu_usage().unwrap()).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    else {
                        b.memory_usage().unwrap().partial_cmp(&a.memory_usage().unwrap()).unwrap_or(std::cmp::Ordering::Equal)
                    }
                })
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
    pub fn get_idx(&mut self, pid: u32) -> Option<usize> {
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
    use crate::process::process_list_items::ProcessListItems;
    use crate::process::process_list_item::{ProcessListItem, CpuInfo};
    use crate::components::ListSortOrder;

    #[test]
    fn test_filter_update() {
        let pid: u32 = 0;
        let name: String = String::from("process_1");
        let cpu_usage: f32 = 0.0;
        let cpu_info = CpuInfo::new(pid, name, cpu_usage);
        let process_list_item_1 = ProcessListItem::Cpu(cpu_info.clone());

        let pid: u32 = 1;
        let name: String = String::from("process_2");
        let cpu_usage: f32 = 0.1;
        let cpu_info = CpuInfo::new(pid, name, cpu_usage);
        let process_list_item_2 = ProcessListItem::Cpu(cpu_info.clone());

        let list: Vec<ProcessListItem> = vec![process_list_item_1.clone(), process_list_item_2.clone()];
        let empty_list: Vec<ProcessListItem> = vec![];

        let mut items = ProcessListItems::new(&list);

        // testing filter()
        let filtered_items = items.filter(String::from("process_1"));
        assert_eq!(filtered_items.list_len(), 1);
        assert_eq!(filtered_items.get_item(0), Some(&process_list_item_1));

        // testing get_item()
        let filtered_items = items.filter(String::from("process_3"));
        assert_eq!(filtered_items.list_len(), 0);
        assert_eq!(filtered_items.get_item(0), None);
        assert_eq!(filtered_items.get_item(100), None);

        // testing update() with new empty list
        assert_eq!(items.list_len(), 2);
        assert_eq!(items.get_item(0), Some(&process_list_item_1));
        assert_eq!(items.get_item(1), Some(&process_list_item_2));
        let _ = items.update_items(&empty_list, &ListSortOrder::PidDec);
        assert_eq!(items.list_len(), 0);
        assert_eq!(items.get_item(0), None);
        assert_eq!(items.get_item(1), None);
        
        // testing update with new non-empty list
        assert_eq!(items.list_len(), 0);
        assert_eq!(items.get_item(0), None);
        assert_eq!(items.get_item(1), None);
        let _ = items.update_items(&list, &ListSortOrder::PidDec);
        assert_eq!(items.list_len(), 2);
        assert_eq!(items.get_item(0), Some(&process_list_item_2));
        assert_eq!(items.get_item(1), Some(&process_list_item_1));
    }

    #[test]
    fn test_insert_item_idx() {
        let pid: u32 = 0;
        let name: String = String::from("a");
        let cpu_usage: f32 = 0.05;
        let cpu_info = CpuInfo::new(pid, name, cpu_usage);
        let process_list_item_1 = ProcessListItem::Cpu(cpu_info.clone());

        let pid: u32 = 1;
        let name: String = String::from("b");
        let cpu_usage: f32 = 0.1;
        let cpu_info = CpuInfo::new(pid, name, cpu_usage);
        let process_list_item_2 = ProcessListItem::Cpu(cpu_info.clone());

        let list: Vec<ProcessListItem> = vec![process_list_item_1.clone(), process_list_item_2.clone()];


        let pid: u32 = 2;
        let name: String = String::from("c");
        let cpu_usage: f32 = 0.15;
        let cpu_info = CpuInfo::new(pid, name, cpu_usage);
        let mut items = ProcessListItems::new(&list);
        let process_list_item_3 = ProcessListItem::Cpu(cpu_info.clone());

        assert_eq!(items.list_len(), 2);
        let idx = items.insert_item_idx(&process_list_item_3, &ListSortOrder::UsageDec);
        assert_eq!(idx, 0);

        let idx = items.insert_item_idx(&process_list_item_3, &ListSortOrder::UsageInc);
        assert_eq!(idx, 2);

        let idx = items.insert_item_idx(&process_list_item_3, &ListSortOrder::NameDec);
        assert_eq!(idx, 0);

        let idx = items.insert_item_idx(&process_list_item_3, &ListSortOrder::NameInc);
        assert_eq!(idx, 2);

        let idx = items.insert_item_idx(&process_list_item_3, &ListSortOrder::PidDec);
        assert_eq!(idx, 0);

        let idx = items.insert_item_idx(&process_list_item_3, &ListSortOrder::PidInc);
        assert_eq!(idx, 2);
    }
}