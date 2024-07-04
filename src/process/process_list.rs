
use std::io;

use super::process_list_items::ProcessListItems;
use super::process_list_items::ProcessListItem;
use super::list_iter::ListIterator;

#[derive(Copy, Clone)]
pub enum MoveSelection {
    Up,
    Down,
    MultipleUp,
    MultipleDown,
    Top,
    End,
    //Terminate,
}

#[derive(Default)]
pub struct ProcessList {
    items: ProcessListItems,
    pub selection: Option<usize>,
    // consider adding visual_selection
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
            selection: if list.is_empty() { None } else { Some(0) },
        }
    }

    // pub fn update
    // inputs:
    // new_list: &Vec<ProcessListItem> -- Reference to a Vector of new ProcessListItem's
    //
    pub fn update(&mut self, new_list: &Vec<ProcessListItem>) -> io::Result<()> {
        self.items.update_items(new_list)?;

        //if !new_list.is_empty() && self.selection.is_none() {
        //    self.selection = Some(0);
        //}

        Ok(())
    }

    // pub fn filter
    // inputs:
    //   filter_text: String -- text to filter processes by name
    // outputs:
    //    new ProcessList
    //
    pub fn filter(&self, filter_text: String) -> Self {
        let mut new_self = Self {
            items: self.items.filter(filter_text),
            selection: if self.items.list_items.is_empty() {
                None
            }
            else {
                Some(0)
            }
        };
        new_self
    }

    // pub fn selection -- getter
    //
    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    // pub fn move_selection -- change self.selected_item given a direction
    // inputs:
    //   dir: MoveSelection
    // outputs:
    //   If selection was moved, then True, else False.
    //
    pub fn move_selection(&mut self, dir: MoveSelection) -> bool {
        // update selection
        //
        if self.items.list_items.len() == 0 { self.selection = None }
        if self.selection.is_none() && self.items.list_items.len() > 0 { self.selection = Some(0) }

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
        let items_max = self.items.len().saturating_sub(1);

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
        let items_max = self.items.len().saturating_sub(1);
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

    // currently not using -- will be used for visual_selection when visual_selection is implemented
    pub fn iterate(&self, start_index: usize, max_amount: usize) -> ListIterator<'_> {
        let start = start_index;
        ListIterator::new(self.items.iterate(start, max_amount), self.selection)
    }
}