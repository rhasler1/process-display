use std::cell::Cell;

pub struct VerticalScroll {
    top: Cell<usize>,
    max_top: Cell<usize>,
    inside: bool,
    border: bool,
}

impl VerticalScroll {
    pub const fn new(border: bool, inside: bool) -> Self {
        Self {
            top: Cell::new(0),
            max_top: Cell::new(0),
            border,
            inside,
        }
    }

    pub fn get_top(&self) -> usize {
        self.top.get()
    }

    pub fn reset(&self) {
        self.top.set(0);
    }

    pub fn update(&self, selection: usize, _visual_height: usize) -> usize {
        // for now we are just setting the top of the list to be the selected item
        self.top.set(selection);
        let new_top = selection;
        new_top
    }
}