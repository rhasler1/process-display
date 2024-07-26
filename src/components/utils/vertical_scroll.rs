use std::cell::Cell;
use ratatui::{
    Frame,
    prelude::*,
};
use crate::{components::DrawableComponent, ui::scrollbar::draw_scrollbar};

pub struct VerticalScroll {
    top: Cell<usize>,
    count: Cell<usize>,
    _inside: bool,
    _border: bool,
}

impl VerticalScroll {
    pub const fn new(_border: bool, _inside: bool) -> Self {
        Self {
            top: Cell::new(0),
            count: Cell::new(0),
            _border,
            _inside,
        }
    }

    pub fn get_top(&self) -> usize {
        self.top.get()
    }

    pub fn reset(&self) {
        self.top.set(0);
    }

    pub fn update(&self, selection: usize, selection_count: usize, visual_height: usize) -> usize {
        let new_top = calc_scroll_top(self.get_top(), visual_height, selection);
        self.count.set(selection_count);
        self.top.set(new_top);
        new_top
    }
}

const fn calc_scroll_top(
    current_top: usize,
    visual_height: usize,
    selection: usize,
) -> usize {
    if visual_height == 0 {
        return 0;
    }
    if current_top + visual_height <= selection {
        return selection;
    }
    else if current_top > selection {
        return selection;
    }
    else {
        return current_top;
    }
}

impl DrawableComponent for VerticalScroll {
    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> std::io::Result<()> {
        draw_scrollbar(
            f,
            area,
            self.top.get(),
            self.count.get(),
        );
        Ok(())
    }
}