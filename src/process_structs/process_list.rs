
use std::io;
use crate::process_structs::list_iter::ListIterator;
use super::ListSortOrder;
use super::process_list_items::ProcessListItems;
use super::process_list_item::ProcessListItem;

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
    // new Constructor
    // inputs:
    //   list: &Vec<ProcessListItem> -- Reference to a Vector of ProcessListItem
    // outputs:
    //   new ProcessList
    pub fn new(list: &Vec<ProcessListItem>) -> Self {
        Self {
            items: ProcessListItems::new(list),
            sort: ListSortOrder::CpuUsageInc,
            follow_selection: false,
            selection: if list.is_empty() { None } else { Some(0) },
        }
    }

    // pub fn filter
    // inputs:
    //   filter_text: String -- text to filter processes by name
    // outputs:
    //    new ProcessList
    pub fn filter(&self, filter_text: String) -> Self {
        let new_self = Self {
            items: self.items.filter(filter_text.clone()),
            sort: ListSortOrder::CpuUsageInc,
            follow_selection: false,
            selection:
                if self.items.filter(filter_text.clone()).list_len() > 0 {
                    Some(0)
                }
                else {
                    None
                },
            };
        new_self
    }
    
    // This function returns true if the instance field items is empty, otherwise false.
    pub fn list_is_empty(&self) -> bool {
        self.items.list_len() == 0
    }

    // This function returns the Pid of the selected item, returns None if item cannot
    // be retrieved or selection is None.
    pub fn get_selected_pid(&self) -> Option<u32> {
        if let Some(selection) = self.selection {
            if let Some(item) = self.items.get_item(selection) {
                return Some(item.pid())
            }
            else { return None }
        }
        None
    }

    // This function is called whenever there is a refresh event. The function is responsible for
    // updating the instance items with the parameter new_list.
    pub fn update(&mut self, new_list: &Vec<ProcessListItem>) -> io::Result<()> {
        // Get the selected item, selected_item = Some(item) || None.
        let selected_item: Option<&ProcessListItem> = self.items.get_item(self.selection.unwrap_or_default());

        // Get the selected item's pid, pid = Some(pid) || None.
        let pid: Option<u32> = selected_item.map(|item| item.pid());

        // Update items with new list.
        self.items.update_items(new_list, &self.sort)?;

        // If pid is some then set self.selection = pid, else self.selection = None. IE: If the item being followed
        // is removed from the list on self.items.update_items(), then follow_selection is set to None.
        if self.follow_selection {
            self.selection = pid.and_then(|p| self.items.get_idx(p));
        }
        else {
            // since it is the case that the process list might
            // change in size on update, we need to check if the
            // selection is still in range of the list. If it is not,
            // then set self.selection to the max_idx.
            if let Some(selection) = self.selection {
                let max_idx = self.items.list_len().saturating_sub(1);
                // If the are no items in the list after the update, then set selection to None.
                if self.items.list_len() == 0 {
                    self.selection = None
                }
                // Else if the length of items shrinks in size after the update and the selection is
                // now greater than the max_idx, set selection to max_idx.
                else if selection > max_idx {
                    self.selection = Some(max_idx)
                }
            }
        }

        // If selection is None prior to self.update being called or if selection is set to None because the followed item
        // was removed from the list, then we need to check if the list is non-empty and set selection to Some(0).
        if self.items.list_len() > 0 && self.selection.is_none() {
            self.selection = Some(0);
        }

        Ok(())
    }

    // This function is called when there is a `ListSortOrder` key event. The items length should never change here.
    pub fn sort(&mut self, sort: ListSortOrder) -> io::Result<()> {
        // Get the selected item, selected_item = Some(item) || None.
        let selected_item: Option<&ProcessListItem> = self.items.get_item(self.selection.unwrap_or_default());

        // Get the selected item's pid, pid = Some(pid) || None.
        let pid: Option<u32> = selected_item.map(|item| item.pid());

        // Only sort if the parameter ListSortOrder differs from the instance ListSortOrder.
        if self.sort != sort {
            self.sort = sort.clone();
            self.items.sort_items(&sort)?;
        }

        // If follow selection, then set self.selection to the new index of the selected item's pid.
        if self.follow_selection {
            self.selection = pid.and_then(|p| self.items.get_idx(p));
        }

        Ok(())
    }

    // This function gets the instance selection.
    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    // This function gets an optional reference to the selected process item.
    pub fn selected_item(&self) -> Option<&ProcessListItem> {
        let selected_item: Option<&ProcessListItem> = self.items.get_item(self.selection.unwrap_or_default());
        selected_item
    }

    // This function returns true if follow_selection is true, otherwise false.
    pub fn follow(&self) -> bool {
        self.follow_selection
    }

    // This function is responsible for changing the follow_selection field. If follow_selection is true,
    // then set to false, else set to true.
    pub fn change_follow_selection(&mut self) -> io::Result<()> {
        if self.follow_selection {
            self.follow_selection = false;
        }
        else {
            self.follow_selection = true;
        }
        Ok(())
    }

    // pub fn move_selection -- change self.selected_item given a direction
    // inputs:
    //   dir: MoveSelection
    // outputs:
    //   If selection was moved, then True, else False.
    pub fn move_selection(&mut self, dir: MoveSelection) -> bool {
        self.selection.map_or(false, |selection| {
            let new_index = match dir {
                MoveSelection::Down => self.selection_down(selection, 1),
                MoveSelection::MultipleDown => self.selection_down(selection, 10),
                MoveSelection::Up => self.selection_up(selection, 1),
                MoveSelection::MultipleUp => self.selection_up(selection, 10),
                MoveSelection::End => self.selection_end(selection),
                MoveSelection::Top => self.selection_start(selection),
            };

            // Changed_index is true if index was moved.
            let changed_index = new_index.map(|i| i != selection).unwrap_or_default();

            if changed_index {
                self.selection = new_index;
            }

            // "if changed index is true then new_index should always be some"
            changed_index || new_index.is_some()
        })
    }

    // fn selection_down -- move selection down
    // inputs:
    //   current_index: usize, lines: usize -- how many lines to move down from current_index
    // outputs:
    //   if the selection was moved, then Some(index), else none
    fn selection_down(&self, current_index: usize, lines: usize) -> Option<usize> {
        let mut new_index = current_index;
        let items_max = self.items.list_len().saturating_sub(1);

        'a: for _ in 0..lines {
            if new_index >= items_max {
                break 'a;
            }
            new_index = new_index.saturating_add(1);
        }

        if new_index == current_index {
            None
        }
        else {
            Some(new_index)
        }
    }

    // fn selection_up -- move selection up
    // inputs:
    //   current_index: usize, lines: usize -- how many lines to move up from current_index
    // outputs:
    //   if the selection was moved, then Some(new_index), else None.
    fn selection_up(&self, current_index: usize, lines: usize) -> Option<usize> {
        let mut new_index = current_index;
        // labeling loop `a` to break out of `a` from within nested loop
        'a: for _ in 0..lines {
            if new_index == 0 {
                break 'a;
            }
            new_index = new_index.saturating_sub(1);
        }

        if new_index == current_index { None }
        else { Some(new_index) }
    }

    // fn selection_end -- move selection to last item in list
    // inputs:
    //   current_index: usize
    // outputs:
    //   If selection was moved, then Some(new_index), else None.
    fn selection_end(&self, current_index: usize) -> Option<usize> {
        let items_max = self.items.list_len().saturating_sub(1);
        let new_index = items_max;

        if new_index == current_index { None }
        else { Some(new_index) }

    }

    // fn selection_start -- move selection to first item in list
    // inputs:
    //   current_index: usize
    // outputs:
    //   If selection was moved, then Some(0), else None.
    fn selection_start(&self, current_index: usize) -> Option<usize> {
        if current_index == 0 { None }
        else { Some(0) }
    }

    // pub fn iterate
    pub fn iterate(&self, start_index: usize, max_amount: usize) -> ListIterator<'_> {
        let start = start_index;
        ListIterator::new(self.items.iterate(start, max_amount), self.selection)
    }
}

#[cfg(test)]
mod test {
    use super::ListSortOrder;
    use crate::process_structs::process_list_item::ProcessListItem;
    use crate::process_structs::process_list::ProcessList;
    use super::MoveSelection;

    #[test]
    fn test_constructors() {
        // Default constructor.
        let empty_instance = ProcessList::default();
        assert!(empty_instance.list_is_empty());
        assert_eq!(empty_instance.selection(), None);

        // New constructor.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let instance = ProcessList::new(&items);
        assert!(!instance.list_is_empty());
        assert_eq!(instance.selection(), Some(0));

        // Filter constructor case 1.
        let filter_string = String::from("c");
        let filter_instance = instance.filter(filter_string);
        assert!(filter_instance.list_is_empty());
        assert_eq!(filter_instance.selection(), None);

        // Filter constructor case 2.
        let filter_string = String::from("b");
        let filter_instance = instance.filter(filter_string);
        assert!(!filter_instance.list_is_empty());
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
        assert!(instance.list_is_empty());
        assert!(instance.selection().is_none());

        // Update with non-empty list of items.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        let item_2 = ProcessListItem::new(3, String::from("c"), 3.0, 3);
        let new_items = vec![item_2];
        let _ = instance.update(&new_items);
        assert!(!instance.list_is_empty());
        assert_eq!(instance.selection(), Some(0));

        // Update with empty list of items and follow_selection set to true.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        let _ = instance.change_follow_selection();
        let empty_items = vec![];
        let _ = instance.update(&empty_items);
        assert!(instance.list_is_empty());
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
        assert!(!instance.list_is_empty());
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
        assert!(!instance.list_is_empty());
        assert_eq!(instance.selection(), Some(0));         
    }

    #[test]
    fn test_sort() {
        // Test sort when follow_selection = false.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        assert!(!instance.follow());
        assert_eq!(instance.selection(), Some(0));
        let _ = instance.sort(ListSortOrder::CpuUsageDec);
        assert_eq!(instance.selection(), Some(0));

        // Test sort when follow_selection = true.
        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        let _ = instance.change_follow_selection();
        assert!(instance.follow());
        assert_eq!(instance.selection(), Some(0));
        let _ = instance.sort(ListSortOrder::CpuUsageDec);
        assert_eq!(instance.selection(), Some(1));
    }

    #[test]
    fn test_selection() {
        let mut empty_instance = ProcessList::default();
        assert_eq!(empty_instance.move_selection(MoveSelection::Down), false);
        assert_eq!(empty_instance.selection(), None);

        let item_0 = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        let item_1 = ProcessListItem::new(2, String::from("b"), 2.0, 2);
        let items = vec![item_0, item_1];
        let mut instance = ProcessList::new(&items);
        assert_eq!(instance.selection(), Some(0));
        assert_eq!(instance.move_selection(MoveSelection::Down), true);
        assert_eq!(instance.move_selection(MoveSelection::Down), false);
        assert_eq!(instance.selection(), Some(1));

        assert_eq!(instance.move_selection(MoveSelection::Up), true);
        assert_eq!(instance.move_selection(MoveSelection::Up), false);
        assert_eq!(instance.selection(), Some(0));

        assert_eq!(instance.move_selection(MoveSelection::End), true);
        assert_eq!(instance.move_selection(MoveSelection::End), false);
        assert_eq!(instance.selection(), Some(1));

        assert_eq!(instance.move_selection(MoveSelection::Top), true);
        assert_eq!(instance.move_selection(MoveSelection::Top), false);
        assert_eq!(instance.selection(), Some(0));

        assert_eq!(instance.move_selection(MoveSelection::MultipleDown), true);
        assert_eq!(instance.move_selection(MoveSelection::MultipleDown), false);
        assert_eq!(instance.selection(), Some(1));

        assert_eq!(instance.move_selection(MoveSelection::MultipleUp), true);
        assert_eq!(instance.move_selection(MoveSelection::MultipleUp), false);
        assert_eq!(instance.selection(), Some(0));
    }
}