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
}