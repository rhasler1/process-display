use crate::components::MoveSelection;

pub struct SelectionState {
    pub selection: Option<usize>,
    pub follow_selection: bool,
}

impl SelectionState {
    pub fn new(idx: Option<usize>) -> Self {
        Self {
            selection: idx,
            follow_selection: false,
        }
    }

    pub fn set_selection(&mut self, idx: Option<usize>) {
        self.selection = idx;
    }

    pub fn set_follow(&mut self, follow: bool) {
        self.follow_selection = follow;
    }

    pub fn move_selection(&mut self, dir: MoveSelection, len: usize) {
        if let Some(selection) = self.selection {
            let new_idx = match dir {
                MoveSelection::Down => self.selection_down(selection, 1, len),
                MoveSelection::MultipleDown => self.selection_down(selection, 10, len),
                MoveSelection::Up => self.selection_up(selection, 1),
                MoveSelection::MultipleUp => self.selection_up(selection, 10),
                MoveSelection::Bottom => self.selection_bottom(selection, len),
                MoveSelection::Top => self.selection_top(selection),       
            };

            self.selection = new_idx;
        }
    }

    fn selection_down(&self, current_idx: usize, lines: usize, len: usize) -> Option<usize> {
        let mut new_idx = current_idx;
        let max_idx = len.saturating_sub(1);

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

    fn selection_bottom(&self, _current_idx: usize, len: usize) -> Option<usize> {
        let max_idx = len.saturating_sub(1);

        Some(max_idx)
    }

    fn selection_top(&self, _current_idx: usize) -> Option<usize> {
        let min_idx = 0;

        Some(min_idx)
    }
}