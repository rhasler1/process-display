use ratatui::{
    Frame,
    prelude::*,
    widgets::*,
};

pub fn draw_scrollbar(f: &mut Frame, area: Rect, top: usize, count: usize) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
        .style(Color::White);
        //.thumb_style(Color::);

    let mut scrollbar_state = ScrollbarState::new(count).position(top);

    f.render_stateful_widget(scrollbar,
        area.inner(&Margin {
        vertical: 1,
        horizontal: 0,
    }),
    &mut scrollbar_state)
}