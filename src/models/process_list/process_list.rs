use super::process_item::ProcessItem;
use super::ProcessItemSortOrder;
use crate::components::utils::selection::SelectionState;
use crate::components::utils::MoveSelection;
use crate::components::sysinfo_wrapper::SysInfoWrapper;
use crate::models::process_list::process_item_iter::ProcessItemIterator;

// A ProcessList can be constructed as an "unfiltered list" with the new(&SysInfoWrapper) constructor
// or as a "filtered list" with filter(&str) constructor.
// Additionally, the filter(&str) constructor uses an existing instance of a ProcessList,
// thus the only constructor that interacts with the sysinfo backend is new(&SysInfoWrapper).
// If filter = None, then the List is Unfiltered, if Some() then the List if Filtered.
pub struct ProcessList {
    processes: Vec<ProcessItem>,
    sort: ProcessItemSortOrder,
    selection_state: SelectionState,
    filter: Option<String>,
}

impl ProcessList {
    // constructor
    pub fn new(sysinfo: &SysInfoWrapper) -> Self {
        let mut processes: Vec<ProcessItem> = Vec::new();
        // sysinfo.get_processes(&mut vec) populates argument Vec with system processes. See /components/sysinfo_wrapper.rs for implementation details.
        sysinfo.get_processes(&mut processes);

        // setting defaults explicitly
        let sort: ProcessItemSortOrder = ProcessItemSortOrder::CpuUsageDec;
        let selection_state: SelectionState = if processes.len() > 0 { SelectionState::new(Some(0)) } else { SelectionState::new(None) };
        let filter: Option<String> = None;

        Self {
            processes,
            sort,
            selection_state,
            filter,
        }
    }

    // filter constructor
    pub fn filter(&self, filter_text: &str) -> Self {
        // filtering by process name--case insensitive
        let processes: Vec<ProcessItem> = self.processes
            .iter()
            .filter(|item| {
                item.name().to_lowercase().contains(&filter_text.to_lowercase())
            })
            .cloned()
            .collect();

        // setting defaults explicitly
        let sort: ProcessItemSortOrder = ProcessItemSortOrder::CpuUsageDec;
        let selection_state: SelectionState = if processes.len() > 0 { SelectionState::new(Some(0)) } else { SelectionState::new(None) };
        let filter: Option<String> = Some(String::from(filter_text));

        Self {
            processes,
            sort,
            selection_state,
            filter,
        }
    }


    pub fn update(&mut self, sysinfo: &SysInfoWrapper) {
        // storing reference to selected item and deep copy of it's PID before updating processes
        let selection_item: Option<&ProcessItem> = self.processes.get(self.selection_state.selection.unwrap_or_default());
        let selection_pid: Option<u32> = selection_item.map(|item| item.pid());

        // get new processes
        sysinfo.get_processes(&mut self.processes);
        // filter if this is a "filtered list"
        if let Some(filter) = &self.filter {
            self.processes.retain(|item| {
                item.name().to_lowercase().contains(&filter.to_lowercase())
            });
        }

        // return if update resulted in no processes
        if self.processes.len() == 0 {
            self.selection_state.set_selection(None);
            self.selection_state.set_follow(false);
            return
        }

        // sort order is lost when getting new processes
        self.sort(&self.sort.clone());

        // set selection after update
        let selection = if self.selection_state.follow_selection {
            selection_pid.and_then(|p| {
                self.processes
                    .iter()
                    .position(|item| item.pid() == p)
            })
        }
        else {
            if let Some(selection) = self.selection_state.selection {
                // check upper bound (lowerbound is effectively checked when checking for NO processes)
                let max_idx = self.processes.len().saturating_sub(1);
                if selection > max_idx {
                    Some(max_idx)
                }
                else {
                    Some(selection)
                }
            }
            else {
                None
            }
        };

        self.selection_state.set_selection(selection);
    }

    pub fn sort(&mut self, sort: &ProcessItemSortOrder) {
        let selection_item: Option<&ProcessItem> = self.processes.get(self.selection_state.selection.unwrap_or_default());
        let selection_pid: Option<u32> = selection_item.map(|item| item.pid());

        // mapping variants to corresponding static comparator functions (see /process_item.rs)
        match sort {
            ProcessItemSortOrder::PidInc => self.processes.sort_by(ProcessItem::cmp_pid_inc),
            ProcessItemSortOrder::PidDec => self.processes.sort_by(ProcessItem::cmp_pid_dec),
            ProcessItemSortOrder::NameInc => self.processes.sort_by(ProcessItem::cmp_name_inc),
            ProcessItemSortOrder::NameDec => self.processes.sort_by(ProcessItem::cmp_name_dec),
            ProcessItemSortOrder::CpuUsageInc => self.processes.sort_by(ProcessItem::cmp_cpu_inc),
            ProcessItemSortOrder::CpuUsageDec => self.processes.sort_by(ProcessItem::cmp_cpu_dec),
            ProcessItemSortOrder::MemoryUsageInc => self.processes.sort_by(ProcessItem::cmp_mem_inc),
            ProcessItemSortOrder::MemoryUsageDec => self.processes.sort_by(ProcessItem::cmp_mem_dec),
        }

        // assign field to new sort variant
        self.sort = sort.clone();

        // update selection if following
        if self.selection_state.follow_selection {
            self.selection_state.selection = selection_pid.and_then(|p| {
                self.processes
                    .iter()
                    .position(|item| item.pid() == p)
            });
        }
    }

    pub fn move_selection(&mut self, dir: MoveSelection) {
        self.selection_state.move_selection(dir, self.processes.len());
    }

    pub fn toggle_follow_selection(&mut self) {
        if self.processes.len() == 0 {
            // nothing to follow: do not toggle, enforce false
            self.selection_state.set_follow(false);
        }
        else {
            self.selection_state.set_follow(!self.selection_state.follow_selection);
        }
    }

    // GETTERS
    pub fn is_follow_selection(&self) -> bool {
        self.selection_state.follow_selection
    }

    pub fn selection(&self) -> Option<usize> {
        self.selection_state.selection
    }

    pub fn is_empty(&self) -> bool {
        self.processes.len() == 0
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }

    pub fn selection_item(&self) -> Option<&ProcessItem> {
        if let Some(selection) = self.selection_state.selection {
            let selection_item = self.processes.get(selection);
            return selection_item
        }

        None
    }

    pub fn selection_pid(&self) -> Option<u32> {
        if let Some(selection) = self.selection_state.selection {
            if let Some(item) = self.processes.get(selection) {
                return Some(item.pid())
            }
            else {
                return None
            }
        }

        None
    }

    pub fn sort_order(&self) -> &ProcessItemSortOrder {
        &self.sort
    }

    pub fn iterate(&self, start_idx: usize, max_amount: usize) -> ProcessItemIterator {
        ProcessItemIterator::new(&self.processes, self.selection_state.selection, start_idx, max_amount)
    }
}

/*
#[cfg(test)]
mod test {
    use crate::models::process_list::{ProcessList, ListSortOrder, MoveSelection};
    use crate::models::process_list_item::ProcessListItem;

    #[test]
    fn test_constructors() {
        // Default constructor.
        let empty_instance = ProcessList::default();
        assert!(empty_instance.is_empty());
        assert_eq!(empty_instance.selection(), None);

        // New constructor.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let instance = ProcessList::new(items);
        assert!(!instance.is_empty());
        assert_eq!(instance.selection(), Some(0));

        // Filter constructor case 1.
        let filter_string = String::from("c");
        let filter_instance = instance.filter(&filter_string);
        assert!(filter_instance.is_empty());
        assert_eq!(filter_instance.selection(), None);

        // Filter constructor case 2.
        let filter_string = String::from("b");
        let filter_instance = instance.filter(&filter_string);
        assert!(!filter_instance.is_empty());
        assert_eq!(filter_instance.selection(), Some(0));
    }

    #[test]
    fn test_update() {
        // Update with empty list of items.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(items);
        let empty_items = vec![];
        let _ = instance.update(empty_items);
        assert!(instance.is_empty());
        assert!(instance.selection().is_none());

        // Update with non-empty list of items.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(items);
        let item_2 = ProcessListItem::new(3, String::from("c"), 3.0, 3, 0, 10, 10, String::from("test"), String::from("test"));
        let new_items = vec![item_2];
        let _ = instance.update(new_items);
        assert!(!instance.is_empty());
        assert_eq!(instance.selection(), Some(0));

        // Update with empty list of items and follow_selection set to true.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(items);
        let _ = instance.toggle_follow_selection();
        let empty_items = vec![];
        let _ = instance.update(empty_items);
        assert!(instance.is_empty());
        assert!(instance.selection().is_none());

        // Update with non-empty list of items and follow_selection set to true case 1.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(items);
        let _ =  instance.toggle_follow_selection();
        let item_2 = ProcessListItem::new(3, String::from("c"), 3.0, 3, 0, 10, 10, String::from("test"), String::from("test"));
        let new_items = vec![item_2];
        let _ = instance.update(new_items);
        assert!(!instance.is_empty());
        assert_eq!(instance.selection(), Some(0));

        // Update with non-empty list of items and follow_selection set to true case 2.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(items);
        let _ =  instance.toggle_follow_selection();
        let item_2 = ProcessListItem::new(2, String::from("b"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let item_3 = ProcessListItem::new(3, String::from("c"), 3.0, 3, 0, 10, 10, String::from("test"), String::from("test"));
        let new_items = vec![item_2, item_3];
        let _ = instance.update(new_items);
        assert!(!instance.is_empty());
        assert_eq!(instance.selection(), Some(0));         
    }

    #[test]
    fn test_sort() {
        // Test sort when follow_selection = false.
        let item_0 = ProcessListItem::new(1, String::from("a"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_1, item_0];
        let mut instance = ProcessList::new(items);
        assert!(instance.sort == ListSortOrder::CpuUsageDec);
        assert!(!instance.is_follow_selection());
        assert_eq!(instance.selection(), Some(0));
        let _ = instance.sort(&ListSortOrder::CpuUsageInc);
        assert_eq!(instance.selection(), Some(0));


        let item_0 = ProcessListItem::new(1, String::from("a"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(items);
        assert!(instance.sort == ListSortOrder::CpuUsageDec);
        let _ = instance.toggle_follow_selection();
        assert!(instance.is_follow_selection());
        assert_eq!(instance.selection(), Some(0));
        let _ = instance.sort(&ListSortOrder::CpuUsageInc);
        assert_eq!(instance.selection(), Some(1));
    }

    #[test]
    fn test_selection() {
        let mut empty_instance = ProcessList::default();
        empty_instance.move_selection(MoveSelection::Down);
        assert_eq!(empty_instance.selection(), None);

        let item_0 = ProcessListItem::new(1, String::from("a"), 2.0, 2, 0, 10, 10, String::from("test"), String::from("test"));
        let item_1 = ProcessListItem::new(2, String::from("b"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(items);
        assert_eq!(instance.selection(), Some(0));
        instance.move_selection(MoveSelection::Down);
        instance.move_selection(MoveSelection::Down);
        assert_eq!(instance.selection(), Some(1));

        instance.move_selection(MoveSelection::Up);
        instance.move_selection(MoveSelection::Up);
        assert_eq!(instance.selection(), Some(0));

        instance.move_selection(MoveSelection::End);
        instance.move_selection(MoveSelection::End);
        assert_eq!(instance.selection(), Some(1));

        instance.move_selection(MoveSelection::Top);
        instance.move_selection(MoveSelection::Top);
        assert_eq!(instance.selection(), Some(0));

        instance.move_selection(MoveSelection::MultipleDown);
        instance.move_selection(MoveSelection::MultipleDown);
        assert_eq!(instance.selection(), Some(1));

        instance.move_selection(MoveSelection::MultipleUp);
        instance.move_selection(MoveSelection::MultipleUp);
        assert_eq!(instance.selection(), Some(0));
    }
}
*/