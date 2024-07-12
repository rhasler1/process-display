use std::io;
use crate::components::ListSortOrder;
use super::list_items_iter::ListItemsIterator;

// information pertinent to system cpu
//
#[derive(Clone, Debug)]
pub struct CpuInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
}

impl CpuInfo {
    pub fn new(pid: u32, name: String, cpu_usage: f32) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
        }
    }
}

// information pertinent to system memory
//
#[derive(Clone, Debug)]
pub struct MemoryInfo {
    pub pid: u32,
    pub name: String,
    pub memory_usage: u64,
}

// ProcessListItem can be of Cpu or Memory
// TODO: add Energy, Disk, and Network
//
#[derive(Clone, Debug)]
pub enum ProcessListItem {
    Cpu(CpuInfo),
    Memory(MemoryInfo),
}

impl ProcessListItem {
    pub const fn is_cpu(&self) -> bool {
        matches!(self, Self::Cpu { .. })
    }

    pub const fn is_memory(&self) -> bool {
        matches!(self, Self::Memory { .. })
    }

    // pub fn is_match
    // inputs:
    //   filter_text: &str -- text to match the ProcessListItem::Variant::name with (all Variants will contain a name field)
    // outputs:
    //   bool -- true if match, false if no match.
    //
    pub fn is_match(&self, filter_text: &str) -> bool {
        match self {
            Self::Cpu(cpu) => cpu.name.contains(filter_text),
            Self::Memory(memory) => memory.name.contains(filter_text),
        }
    }

    // pub fn name -- getter
    //
    pub fn name(&self) -> String {
        match self {
            Self::Cpu(cpu) => cpu.name.clone(),
            Self::Memory(memory) => memory.name.clone(),
        }
    }

    // pub fn pid -- getter
    //
    pub fn pid(&self) -> u32 {
        match self {
            Self::Cpu(cpu) => cpu.pid.clone(),
            Self::Memory(memory) => memory.pid.clone(),
        }
    }

    // pub fn cpu_usage -- getter
    // output:
    //   Some(cpu_usage: f32) || None
    //
    pub fn cpu_usage(&self) -> Option<f32> {
        match self {
            Self::Cpu(cpu) => Some(cpu.cpu_usage.clone()),
            Self::Memory(_memory) => None,
        }
    }

    // pub fn memory_usage -- getter
    // outputs:
    //   Some(memory_usage: u64) || None
    //
    pub fn memory_usage(&self) -> Option<u64> {
        match self {
            Self::Cpu(_cpu) => None,
            Self::Memory(memory) => Some(memory.memory_usage.clone()),
        }
    }
}

// PartialEq implemented to update Items in ProcessListItems
//
impl PartialEq for ProcessListItem {
    fn eq(&self, other: &Self) -> bool {
        // comparing processes pid's -- the pid is a unique identifier for all variants of ProcessListItem
        return self.pid().eq(&other.pid())
    }
}

#[derive(Default, Clone)]
pub struct ProcessListItems {
    // vector of type ProcessListItem
    pub list_items: Vec<ProcessListItem>,
}

impl ProcessListItems {
    // New Constructor
    // inputs:
    //   list: Vec<ProcessListItem> -- Vector containing any variant of ProcessListItem
    //
    pub fn new(list: &Vec<ProcessListItem>) -> Self {
        Self {
            list_items: Self::create_items(list),
        }
    }

    // fn create_items
    // inputs:
    //   list: &Vec<ProcessListItem> -- Reference to a vector containing any variant of ProcessListItem
    // outputs:
    //   list: Vec<ProcessListItem> -- New vector containing entries from argument
    //
    fn create_items(list: &Vec<ProcessListItem>) -> Vec<ProcessListItem> {
        let mut items = Vec::with_capacity(list.len());
        for e in list {
            items.push(e.clone());
        }
        return items;
    }

    // pub fn filter
    //
    pub fn filter(&self, filter_text: String) -> Self {
        Self {
            list_items: self
                .list_items
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

    // pub fn update_items
    // inputs:
    //   new_list: &Vec<ProcessListItem>
    // outputs:
    //   updates self.list_items with argument new_list
    //   Ok(())
    //
    pub fn update_items(&mut self, new_list: &Vec<ProcessListItem>, sort: &ListSortOrder) -> io::Result<()> {
        for e in new_list {
             // 1. if the new list contains an entry not in the old list, then add entry to old list.
             // - need to improve the method for pushing an item onto a list,
             // - ie: if the list is sorted push the entry into it's sorted position
             //
            if !self.list_items.contains(e) {
                // get the index to insert the new item
                // ideally log(n) insertion time into a sorted list
                //
                let idx = self.insert_item_idx(e, sort);
                // insert new item
                //
                self.list_items.insert(idx, e.clone())
            }
            // 2. if the new list contains updated information for a process in the old list, then update old list entry.
            //
            else if let Some(old_item) = self.list_items.iter_mut().find(|item| item == &e) {
                *old_item = e.clone();
            }
        }
        // 3. if the old list contains an entry not in the new list, then remove entry from old list.
        //
        self.list_items.retain(|item| new_list.contains(item));

        // the list might become unsorted if sorting by usage-- an items usage value
        // might change on refresh events, whereas an item's name or pid will not.
        //
        if *sort == ListSortOrder::UsageInc || *sort == ListSortOrder::UsageDec {
            self.sort_items(sort)?;
        }
        
        Ok(())
    }

    // fn insert_item_idx -- get the index to insert the argument item into a list
    //
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

    // pub fn sort_items
    //
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

    // pub fn get_idx
    // inputs:
    //   pid: u32 -- pid to search index for
    // outputs:
    //   Ok(idx)
    //
    pub fn get_idx(&mut self, pid: u32) -> Option<usize> {
        if let Some(idx) = self.list_items.iter_mut().position(|item| item.pid() == pid) {
            return Some(idx);
        }
        else {
            return None;
        }
    }

    pub fn get_item(&self, idx: usize) -> Option<&ProcessListItem> {
        let max_idx = self.list_len().saturating_sub(1);
        if self.list_len() == 0 || idx > max_idx {
            return None
        }
        let item = self.list_items.get(idx);
        item
    }

    // pub fn len -- getter to self.list_items.len()
    //
    pub fn list_len(&self) -> usize {
        self.list_items.len()
    }

    // pub const fn iterate
    //
    pub const fn iterate(&self, start: usize, max_amount: usize) -> ListItemsIterator<'_> {
        ListItemsIterator::new(self, start, max_amount)
    }
}

#[cfg(test)]
mod test {
    use crate::process::process_list_items::ProcessListItems;
    use std::vec;
    use super::ProcessListItem;
    use super::CpuInfo;
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