use std::io;

use super::list_items_iter::ListItemsIterator;

// information pertinent to system cpu
//
#[derive(Clone)]
pub struct CpuInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
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
            list_items: Self::create_items(list)
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
    pub fn update_items(&mut self, new_list: &Vec<ProcessListItem>) -> io::Result<()> {
        for e in new_list {
             // 1. if the new list contains an entry not in the old list, then add entry to old list.
             //
            if !self.list_items.contains(e) { self.list_items.push(e.clone()) }
            // 2. if the new list contains updated information for a process in the old list, then update old list entry.
            //
            else if let Some(old_item) = self.list_items.iter_mut().find(|item| item == &e) {
                *old_item = e.clone();
            }
        }
        // 3. if the old list contains an entry not in the new list, then remove entry from old list.
        //
        self.list_items.retain(|item| new_list.contains(item));
        Ok(())
    }

    // pub fn filter
    //
    pub fn filter(&self, filter_text: String) -> Self {
        Self {
            list_items: self
                .list_items
                .iter()
                .filter(|item| {
                    item.is_cpu() || item.is_memory() || item.is_match(&filter_text)
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