use anyhow::Result;
use std::cell::Cell;
use ratatui::{
    Frame,
    prelude::*,
    widgets::*,
};
use crate::components::DrawableComponent;

pub struct VerticalScroll {
    top: Cell<usize>,
    count: Cell<usize>,
}

impl VerticalScroll {
    pub const fn new() -> Self {
        Self {
            top: Cell::new(0),
            count: Cell::new(0),
        }
    }

    pub fn get_top(&self) -> usize {
        self.top.get()
    }

    pub fn reset(&self) {
        self.top.set(0);
    }

    pub fn update(
        &self,
        selection: usize,
        selection_len: usize,
        visual_height: usize)
        -> usize {
            let new_top = calc_scroll_top(self.get_top(), visual_height, selection);
            self.count.set(selection_len);
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

    let padding = visual_height / 2;
    let min_top = selection.saturating_sub(padding);

    if selection < current_top + padding {
        min_top
    }
    else if selection >= current_top + visual_height - padding {
        min_top
    }
    else {
        current_top
    }
}

impl DrawableComponent for VerticalScroll {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()> {
        draw_scrollbar(
            f,
            area,
            self.top.get(),
            self.count.get(),
            focused,
        );
        Ok(())
    }
}

fn draw_scrollbar(f: &mut Frame, area: Rect, top: usize, count: usize, focused: bool) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
        .style({
            if focused {
                Color::LightGreen
            }
            else {
                Color::DarkGray
            }
        });

    let mut scrollbar_state = ScrollbarState::new(count).position(top);
    
    f.render_stateful_widget(
        scrollbar,
        area.inner(
            &Margin {
                vertical: 1,
                horizontal: 0,
            }
        ),
        &mut scrollbar_state
    );
}