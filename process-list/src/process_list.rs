use crate::process_list_items::ProcessListItems;
use crate::process_list_item::ProcessListItem;
use crate::list_iter::ListIterator;

#[derive(PartialEq, Clone, Default)]
pub enum ListSortOrder {
    PidInc,
    PidDec,
    NameInc,
    NameDec,
    CpuUsageInc,
    #[default] CpuUsageDec,
    MemoryUsageInc,
    MemoryUsageDec,
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
    pub fn new(list: Vec<ProcessListItem>) -> Self {
        assert!(!list.is_empty());

        Self {
            items: ProcessListItems::new(list),
            sort: ListSortOrder::default(),
            follow_selection: false,
            selection: Some(0),
        }
    }

    pub fn filter(&self, filter_text: &str) -> Self {
        let items = self.items.filter(filter_text);
        let len = items.items_len();

        Self {
            items,
            sort: ListSortOrder::default(),
            follow_selection: false,
            selection: if len > 0 {
                Some(0)
            }
            else {
                None
            },
        }
    }

    pub fn update(&mut self, new_list: Vec<ProcessListItem>) {
        let selected_item: Option<&ProcessListItem> = self.items.get_item_ref(self.selection.unwrap_or_default());
        let pid: Option<u32> = selected_item.map(|item| item.pid());

        self.items.update_items(new_list);
        self.items.sort_items(&self.sort);

        if self.items.items_len() == 0 {
            self.selection = None;
            return
        }

        if self.follow_selection {
            self.selection = pid.and_then(|p| self.items.get_idx(p));
        }
        else {
            if let Some(selection) = self.selection {
                let max_idx = self.items.items_len().saturating_sub(1);
                if selection > max_idx {
                    self.selection = Some(max_idx)
                }
            }
        }

        if self.selection.is_none() {
            self.selection = Some(0)
        }
    }

    pub fn sort(&mut self, sort: &ListSortOrder) {
        let selected_item: Option<&ProcessListItem> = self.items.get_item_ref(self.selection.unwrap_or_default());

        let pid: Option<u32> = selected_item.map(|item| item.pid());

        self.items.sort_items(sort);

        self.sort = sort.clone();

        if self.follow_selection {
            self.selection = pid.and_then(|p| self.items.get_idx(p));
        }
    }

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

    fn selection_end(&self, _current_idx: usize) -> Option<usize> {
        let max_idx = self.items.items_len().saturating_sub(1);

        Some(max_idx)

    }

    fn selection_start(&self, _current_idx: usize) -> Option<usize> {
        let min_idx = 0;

        Some(min_idx)
    }

    pub fn toggle_follow_selection(&mut self) {
        self.follow_selection = !self.follow_selection
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

    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    // gets reference to selected process list item
    pub fn selected_item(&self) -> Option<&ProcessListItem> {
        if let Some(selection) = self.selection {
            let selected_item = self.items.get_item_ref(selection);
            return selected_item
        }
        None
    }

    // gets pid of selected process list item
    pub fn selected_pid(&self) -> Option<u32> {
        if let Some(selection) = self.selection {
            if let Some(item) = self.items.get_item_ref(selection) {
                return Some(item.pid())
            }
            else {
                return None
            }
        }
        None
    }

    pub fn get_sort_order(&self) -> &ListSortOrder {
        &self.sort
    }

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
