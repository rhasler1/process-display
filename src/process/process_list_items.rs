use std::io;

use crate::components::ListSortOrder;

use super::list_items_iter::ListItemsIterator;

// information pertinent to system cpu
//
#[derive(Clone)]
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
#[derive(Clone)]
pub struct MemoryInfo {
    pub pid: u32,
    pub name: String,
    pub memory_usage: u64,
}

// ProcessListItem can be of Cpu or Memory
// TODO: add Energy, Disk, and Network
//
#[derive(Clone)]
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

// Contains a Vector of ProcessListItem
//
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

        // seeing if this 'fixes' sort by cpu usage
        // the list might become unsorted if sorting by usage, usage values update
        // every refresh event...
        // TODO: clean this idea up
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
            _ => self.list_items.len() // if sort is None, return the index to the end of the list
        }
    }

    // how can items be sorted?
    // lets support by usage, by name, by pid
    // the goal is to only call this function when the sort type changes, I
    // do not want to sort the items every time a new item is added, the item
    // should be added to it's correct position given the ListSortOrder by another function
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

    // pub fn len -- getter to self.list_items.len()
    //
    pub fn len(&self) -> usize {
        self.list_items.len()
    }

    // pub const fn iterate -- currently not using (will need if implementing visual selection)
    //
    pub const fn iterate(&self, start: usize, max_amount: usize) -> ListItemsIterator<'_> {
        ListItemsIterator::new(self, start, max_amount)
    }
}