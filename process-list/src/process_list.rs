use crate::process_list_items::ProcessListItems;
use crate::process_list_item::ProcessListItem;
use crate::list_iter::ListIterator;

#[derive(PartialEq, Clone)]
pub enum ListSortOrder {
    PidInc,
    PidDec,
    NameInc,
    NameDec,
    CpuUsageInc,
    CpuUsageDec,
    MemoryUsageInc,
    MemoryUsageDec,
}

impl Default for ListSortOrder {
    fn default() -> Self {
        ListSortOrder::CpuUsageDec
    }
}

#[derive(Copy, Clone)]
pub enum MoveSelection {
    Up,
    Down,
    MultipleUp,
    MultipleDown,
    Top,
    End,
}

#[derive(Default)]
pub struct ProcessList {
    items: ProcessListItems,
    sort: ListSortOrder,
    follow_selection: bool,
    pub selection: Option<usize>,
}

impl ProcessList {
    // constructor
    pub fn new(list: &Vec<ProcessListItem>) -> Self {
        Self {
            items: ProcessListItems::new(list),
            sort: ListSortOrder::default(), // CpuUsageDec
            follow_selection: false,
            selection: if list.is_empty() {
                None
            }
            else {
                Some(0)
            },
        }
    }

    // constructor for filtered list
    pub fn filter(&self, filter_text: &String) -> Self {
        let new_self = Self {
            items: self.items.filter(filter_text),
            sort: ListSortOrder::default(),
            follow_selection: false,
            selection: if self.items.filter(filter_text).items_len() > 0 {
                Some(0)
            }
            else {
                None
            },
        };
        new_self
    }

    // update process list with new list
    pub fn update(&mut self, new_list: &Vec<ProcessListItem>) {
        // get the selected item, either some(item) or None
        let selected_item: Option<&ProcessListItem> = self.items.get_item(self.selection.unwrap_or_default());

        // get the selected item's pid, either some(pid) or None
        let pid: Option<u32> = selected_item.map(|item| item.pid());

        // update items with new list.
        self.items.update_items(new_list);

        // re-sort 
        self.items.sort_items(&self.sort);

        // if list is empty, set selection to None and return
        if self.items.items_len() == 0 {
            self.selection = None;
            return
        }

        // if following, then set selection to selected item's new index
        if self.follow_selection {
            self.selection = pid.and_then(|p| self.items.get_idx(p));
        }
        else {
            // not following, keep the selection where it is, ensuring it is still in bounds
            if let Some(selection) = self.selection {
                let max_idx = self.items.items_len().saturating_sub(1);

                if selection > max_idx {
                    self.selection = Some(max_idx)
                }
            }
        }

        // if selection is none at this point, set to 0
        if self.selection.is_none() {
            self.selection = Some(0)
        }
    }

    // sorts process list by list sort order
    pub fn sort(&mut self, sort: &ListSortOrder) {
        // get the selected item, selected_item = Some(item) || None.
        let selected_item: Option<&ProcessListItem> = self.items.get_item(self.selection.unwrap_or_default());

        // get the selected item's pid, pid = Some(pid) || None.
        let pid: Option<u32> = selected_item.map(|item| item.pid());

        // sort items
        self.items.sort_items(sort);

        // set sort field
        self.sort = sort.clone();

        // if follow selection, then set self.selection to the new index of the selected item's pid.
        if self.follow_selection {
            self.selection = pid.and_then(|p| self.items.get_idx(p));
        }
    }

    // move selection based on MoveSelection variant
    pub fn move_selection(&mut self, dir: MoveSelection) {
        if let Some(selection) = self.selection() {
            let new_idx = match dir {
                MoveSelection::Down => self.selection_down(selection, 1),
                MoveSelection::MultipleDown => self.selection_down(selection, 10),
                MoveSelection::Up => self.selection_up(selection, 1),
                MoveSelection::MultipleUp => self.selection_up(selection, 10),
                MoveSelection::End => self.selection_end(selection),
                MoveSelection::Top => self.selection_start(selection),       
            };
            self.selection = new_idx;
        }
    }

    // calculates and returns new index after moving current down by lines
    fn selection_down(&self, current_idx: usize, lines: usize) -> Option<usize> {
        let mut new_idx = current_idx;
        let max_idx = self.items.items_len().saturating_sub(1);

        'a: for _ in 0..lines {
            if new_idx >= max_idx {
                break 'a;
            }
            new_idx = new_idx.saturating_add(1);
        }

        Some(new_idx)
    }

    // calculates and returns new index after moving current index up by lines
    fn selection_up(&self, current_idx: usize, lines: usize) -> Option<usize> {
        let mut new_idx = current_idx;
        let min_idx = 0;

        'a: for _ in 0..lines {
            if new_idx == min_idx {
                break 'a;
            }
            new_idx = new_idx.saturating_sub(1);
        }

        Some(new_idx)
    }

    // calculates and returns max index 
    fn selection_end(&self, _current_idx: usize) -> Option<usize> {
        let max_idx = self.items.items_len().saturating_sub(1);

        Some(max_idx)

    }

    // returns min index
    fn selection_start(&self, _current_idx: usize) -> Option<usize> {
        let min_idx = 0;

        Some(min_idx)
    }

    // toggle follow selection
    pub fn change_follow_selection(&mut self) {
        if self.follow_selection {
            self.follow_selection = false;
        }
        else {
            self.follow_selection = true;
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.items_len() == 0
    }

    pub fn is_follow_selection(&self) -> bool {
        self.follow_selection
    }

    pub fn len(&self) -> usize {
        self.items.items_len()
    }

    // gets selection index
    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    // gets reference to selected process list item
    pub fn selected_item(&self) -> Option<&ProcessListItem> {
        if let Some(selection) = self.selection {
            let selected_item = self.items.get_item(selection);
            return selected_item
        }
        None
    }

    // gets pid of selected process list item
    pub fn selected_pid(&self) -> Option<u32> {
        if let Some(selection) = self.selection {
            if let Some(item) = self.items.get_item(selection) {
                return Some(item.pid())
            }
            else {
                return None
            }
        }
        None
    }

    // iterator
    pub fn iterate(&self, start_index: usize, max_amount: usize) -> ListIterator<'_> {
        let start = start_index;
        ListIterator::new(self.items.iterate(start, max_amount), self.selection)
    }
}

#[cfg(test)]
mod test {
    use crate::process_list::{ProcessList, ListSortOrder, MoveSelection};
    use crate::process_list_item::ProcessListItem;

    #[test]
    fn test_constructors() {
        // Default constructor.
        let empty_instance = ProcessList::default();
        assert!(empty_instance.is_empty());
        assert_eq!(empty_instance.selection(), None);

        // New constructor.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let instance = ProcessList::new(&items);
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
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        let empty_items = vec![];
        let _ = instance.update(&empty_items);
        assert!(instance.is_empty());
        assert!(instance.selection().is_none());

        // Update with non-empty list of items.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        let item_2 = ProcessListItem::new(3, String::from("c"), 3.0, 3);
        let new_items = vec![item_2];
        let _ = instance.update(&new_items);
        assert!(!instance.is_empty());
        assert_eq!(instance.selection(), Some(0));

        // Update with empty list of items and follow_selection set to true.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        let _ = instance.change_follow_selection();
        let empty_items = vec![];
        let _ = instance.update(&empty_items);
        assert!(instance.is_empty());
        assert!(instance.selection().is_none());

        // Update with non-empty list of items and follow_selection set to true case 1.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        let _ =  instance.change_follow_selection();
        let item_2 = ProcessListItem::new(3, String::from("c"), 3.0, 3);
        let new_items = vec![item_2];
        let _ = instance.update(&new_items);
        assert!(!instance.is_empty());
        assert_eq!(instance.selection(), Some(0));

        // Update with non-empty list of items and follow_selection set to true case 2.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        let _ =  instance.change_follow_selection();
        let item_2 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let item_3 = ProcessListItem::new(3, String::from("c"), 3.0, 3);
        let new_items = vec![item_2, item_3];
        let _ = instance.update(&new_items);
        assert!(!instance.is_empty());
        assert_eq!(instance.selection(), Some(0));         
    }

    #[test]
    fn test_sort() {
        // Test sort when follow_selection = false.
        let item_0 = ProcessListItem::new(1, String::from("a"), 2.0, 2);
        let item_1 = ProcessListItem::new(2, String::from("b"), 1.0, 1);
        let items = vec![item_1, item_0];
        let mut instance = ProcessList::new(&items);
        assert!(instance.sort == ListSortOrder::CpuUsageDec);
        assert!(!instance.is_follow_selection());
        assert_eq!(instance.selection(), Some(0));
        let _ = instance.sort(&ListSortOrder::CpuUsageInc);
        assert_eq!(instance.selection(), Some(0));


        let item_0 = ProcessListItem::new(1, String::from("a"), 2.0, 2);
        let item_1 = ProcessListItem::new(2, String::from("b"), 1.0, 1);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        assert!(instance.sort == ListSortOrder::CpuUsageDec);
        let _ = instance.change_follow_selection();
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

        let item_0 = ProcessListItem::new(1, String::from("a"), 2.0, 2);
        let item_1 = ProcessListItem::new(2, String::from("b"), 1.0, 1);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
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