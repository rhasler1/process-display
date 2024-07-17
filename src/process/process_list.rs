
use std::io;
use crate::components::ListSortOrder;
use crate::process::list_iter::ListIterator;
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
    //
    pub fn new(list: &Vec<ProcessListItem>) -> Self {
        Self {
            items: ProcessListItems::new(list),
            sort: ListSortOrder::UsageInc,
            follow_selection: false,
            selection: if list.is_empty() { None } else { Some(0) },
        }
    }

    // pub fn filter
    // inputs:
    //   filter_text: String -- text to filter processes by name
    // outputs:
    //    new ProcessList
    //
    pub fn filter(&self, filter_text: String) -> Self {
        let new_self = Self {
            items: self.items.filter(filter_text),
            sort: ListSortOrder::UsageInc,
            follow_selection: false,
            selection: if self.items.list_items.is_empty() {
                None
            }
            else {
                Some(0)
            },
        };
        new_self
    }

    pub fn list_is_empty(&self) -> bool {
        self.items.list_len() == 0
    }

    pub fn get_selected_pid(&self) -> Option<u32> {
        if let Some(selection) = self.selection {
            if let Some(item) = self.items.get_item(selection) {
                return Some(item.pid())
            }
            else { return None }
        }
        None
    }

    // pub fn update, note: selection is be updated here.
    // inputs:
    // new_list: &Vec<ProcessListItem> -- Reference to a Vector of new ProcessListItem's
    //
    pub fn update(&mut self, new_list: &Vec<ProcessListItem>) -> io::Result<()> {
        // get the selected item, selected_item = Some(item) || None
        //
        let selected_item: Option<&ProcessListItem> = self.items.get_item(self.selection.unwrap_or_default());

        // get the selected item's pid, pid = Some(pid) || None
        //
        let pid: Option<u32> = selected_item.map(|item| item.pid());

        // update items with new list
        //
        self.items.update_items(new_list, &self.sort)?;

        // if pid is some then set self.selection = pid, else self.selection = None
        //
        if self.follow_selection {
            self.selection = pid.and_then(|p| self.items.get_idx(p));
        }
        else {
            // since it is the case that the process list might
            // change in size on update, we need to check if the
            // selection is still in range of the list. If it is not,
            // then set self.selection to the max_idx.
            //
            if let Some(selection) = self.selection {
                let max_idx = self.items.list_len().saturating_sub(1);
                if selection > max_idx {
                    self.selection = Some(max_idx)
                }
            }
        }

        // if the list is not empty and selection is None, set the selection to be 0.
        //
        if self.items.list_len() > 0 && self.selection.is_none() {
            self.selection = Some(0);
        }

        Ok(())
    }

    // pub function sort, note selection is updated here.
    //
    pub fn sort(&mut self, sort: ListSortOrder) -> io::Result<()> {
        // get the selected item, selected_item = Some(item) || None
        //
        let selected_item: Option<&ProcessListItem> = self.items.get_item(self.selection.unwrap_or_default());

        // get the selected item's pid, pid = Some(pid) || None
        //
        let pid: Option<u32> = selected_item.map(|item| item.pid());

        // sort
        //
        if self.sort != sort {
            self.sort = sort.clone();
            self.items.sort_items(&sort)?;
        }

        // if follow selection, then set self.selection to the new index of the selected item's pid
        //
        if self.follow_selection {
            self.selection = pid.and_then(|p| self.items.get_idx(p));
        }

        Ok(())
    }

    // pub fn selection -- getter
    //
    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    pub fn selected_item(&self) -> Option<&ProcessListItem> {
        let selected_item: Option<&ProcessListItem> = self.items.get_item(self.selection.unwrap_or_default());
        selected_item
    }

    // pub fn follow
    // returns true if self.follow_selection is set to true, else false.
    //
    pub fn follow(&self) -> bool {
        self.follow_selection
    }

    // pub fn change_follow_selection
    // if self.follow_selection is true, then sets to false, else true.
    //
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
    //
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

            // changed_index is true if index was moved
            let changed_index = new_index.map(|i| i != selection).unwrap_or_default();

            if changed_index {
                self.selection = new_index;
            }

            // "if changed index is true then new_index should always be some"
            //panic!();
            changed_index || new_index.is_some()
        })
    }

    // fn selection_down -- move selection down
    // inputs:
    //   current_index: usize, lines: usize -- how many lines to move down from current_index
    // outputs:
    //   if the selection was moved, then Some(index), else none
    //
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
    //
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
    //
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
    //
    fn selection_start(&self, current_index: usize) -> Option<usize> {
        if current_index == 0 { None }
        else { Some(0) }
    }

    // pub fn iterate
    //
    pub fn iterate(&self, start_index: usize, max_amount: usize) -> ListIterator<'_> {
        let start = start_index;
        ListIterator::new(self.items.iterate(start, max_amount), self.selection)
    }
}

#[cfg(test)]
mod test {
}