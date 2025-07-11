// Wrapper around Vec<T> that provides selection state
// Data here is unstructured: To get structured (sorted)
// data use iterators

// In cases where the ListState is dynamically updated
// set 
use crate::components::utils::selection::SelectionState;

pub struct ListState<T> {
    items: T,
    selection_state: SelectionState,
}

impl <T> ListState<Vec<T>> {
    // CONSTRUCTORS::BEGIN
    pub fn new(items: Vec<T>) -> Self {
        let selection_state = if items.is_empty() { SelectionState::new(None) } else { SelectionState::new(Some(0)) };

        Self {
            items,
            selection_state,
        }
    }

    pub fn filter<F>(&self, mut predicate: F) -> Self
    where
        T: Clone,
        F: FnMut(&T) -> bool
    {
        let filtered_items: Vec<T> = self.items
            .iter()
            .filter(|x| predicate(x))
            .cloned()
            .collect();

        let selection_state = if filtered_items.is_empty() {
            SelectionState::new(None)
        }
        else {
            SelectionState::new(Some(0))
        };

        Self {
            items: filtered_items,
            selection_state,
        }
    }
    // CONSTRUCTORS::END

    // MUTATORS::BEGIN
    pub fn replace(&mut self, new_items: Vec<T>) {        
        if new_items.is_empty() {
            self.set_selection(None);
        }
        else {
            self.set_selection(Some(0));
        }

        self.items = new_items;
    }

    pub fn set_selection(&mut self, idx: Option<usize>) {
        let len = self.items.len();

        // items empty case
        if len == 0 {
            self.selection_state.set_selection(None);
            return;
        }

        if let Some(idx) = idx {
            // index within length: set selection to index
            if idx < len {
                self.selection_state.set_selection(Some(idx));
            }
            else {
                // index greater than or equal to length: clamp selection to max_index
                let max_idx = len.saturating_sub(1);
                self.selection_state.set_selection(Some(max_idx));
            }
        }
        else {
            self.selection_state.set_selection(None);
        }
    }

    pub fn select_next(&mut self) {
        let len = self.items.len();
        
        if let Some(idx) = self.selection_state.selection {
            if idx.saturating_add(1) < len {
                self.selection_state.set_selection(Some(idx.saturating_add(1)));
            }
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(idx) = self.selection_state.selection {
            if idx > 0 {
                self.selection_state.set_selection(Some(idx.saturating_sub(1)));
            }
        }
    }

    pub fn select_first(&mut self) {
        if !self.items.is_empty() {
            self.selection_state.set_selection(Some(0));
        }
    }

    pub fn select_last(&mut self) {
        if !self.items.is_empty() {
            self.selection_state.set_selection(Some(self.items.len().saturating_sub(1)));
        }
    }

    pub fn toggle_follow_selection(&mut self) {
        if !self.items.is_empty() {
            // toggle
            self.selection_state.set_follow(!self.selection_state.follow_selection);
        }
        else {
            // no items: must be false
            self.selection_state.set_follow(false);
        }
    }
    // MUTATORS::END

    // GETTERS::BEGIN
    pub fn selected(&self) -> Option<&T> {
        self.selection_state.selection.and_then(|i| self.items.get(i))
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }
    // GETTERS::END

    // ITERATORS
    pub fn iter_sorted<'a, F>(&'a self, mut compare: F) -> impl Iterator<Item = &T> + 'a
    where F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        // generate indices 0..n
        let mut indices: Vec<usize> = (0..self.items.len()).collect();

        // sort indices based on comparing the underlying items
        indices.sort_by(|&i, &j| compare(&self.items[i], &self.items[j]));

        // return an iterator of &T in sorted order
        indices.into_iter().map(move |i| &self.items[i])
    }
}

impl<T: PartialEq> ListState<Vec<T>> {
    pub fn replace_with_follow(&mut self, new_items: Vec<T>)
    where T: Clone
    {
        let old_selection = self.selection_state.selection.and_then(|idx| self.items.get(idx)).cloned();
        self.items = new_items;

        if self.selection_state.follow_selection {
            if let Some(selected_item) = old_selection {
                self.selection_state.set_selection({
                    self.items
                        .iter()
                        .position(|x| *x == selected_item)
                });
            }
            else {
                self.selection_state.set_selection(Some(0))
            }
        }
        else {
            if self.items.is_empty() {
                self.selection_state.set_selection(None)
            }
            else {
                self.selection_state.set_selection(Some(0))
            }
        }
    }
}

impl<T> IntoIterator for ListState<Vec<T>> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list_state_with_items() {
        let items = vec![1, 2, 3];
        let state = ListState::new(items.clone());
        assert_eq!(state.items, items);
        assert_eq!(state.selection_state.selection, Some(0));
    }

    #[test]
    fn test_set_selection_clamps() {
        let items = vec![1, 2, 3];
        let mut state = ListState::new(items);
        state.set_selection(Some(10)); // out of bounds
        assert_eq!(state.selection_state.selection, Some(2));
    }

    #[test]
    fn test_filter() {
        let items = vec![1, 2, 3, 4, 5];
        let state = ListState::new(items);
        let filtered = state.filter(|x| *x % 2 == 0);
        assert_eq!(filtered.items, vec![2, 4]);
        assert_eq!(state.selection_state.selection, Some(0));
    }

    #[test]
    fn test_iter_sorted() {
        let items = vec![3, 1, 2];
        let state = ListState::new(items);
        let sorted: Vec<_> = state.iter_sorted(|a, b| a.cmp(b)).copied().collect();
        assert_eq!(sorted, vec![1, 2, 3]);
    }
}