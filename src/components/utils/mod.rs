pub mod vertical_scroll;
pub mod selection;

#[derive(Copy, Clone)]
pub enum MoveSelection {
    Up,
    Down,
    MultipleUp,
    MultipleDown,
    Top,
    Bottom,
}