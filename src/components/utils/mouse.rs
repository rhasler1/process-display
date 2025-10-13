// interface for components that want to implement
// mouse clicks
// this is dependent on ratatui::rect

use crate::config::MouseConfig;
use ratatui::{layout::Position, prelude::*};

// true if click is within component's area,
// otherwise false
pub fn handle_click_on_component(
    component_area: Rect,
    click_x: u16,
    click_y: u16,
) -> bool
{
    if !component_area.contains(Position { x: click_x, y: click_y }) {
        return false;
    }

    true
}

pub fn handle_click_on_list(
    list_area: Rect,
    list_border_height: u16,
    list_header_height: u16,
    visible_list_height: u16,
    click_x: u16,
    click_y: u16,
) -> Option<u16>
{
    if !list_area.contains(Position { x: click_x, y: click_y }) {
        return None
    }

    let list_top = list_area.top();

    let list_height = list_area.height
        .saturating_sub(list_header_height)
        .saturating_sub(list_border_height);

    let list_upper_bound = list_top
        .saturating_add(list_border_height)
        .saturating_add(list_header_height);

    // this is the lower bound of visible items in the list
    let list_lower_bound = list_top
        .saturating_add(list_border_height)
        .saturating_add(list_header_height)
        .saturating_add(visible_list_height);


    if click_y < list_upper_bound || click_y >= list_lower_bound {
        return None
    }

    // calculate row index
    let row_idx = click_y
        .saturating_sub(list_top)
        .saturating_sub(list_header_height)
        .saturating_sub(list_border_height);

    Some(row_idx)
}