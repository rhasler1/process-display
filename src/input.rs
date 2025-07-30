use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

// wrapping key and mouse inputs to decouple application logic from crossterm
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Key {
    Enter,
    Esc,
    Char(char),
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Tab,
    Unkown,
}

// adapter from crossterm key event type to application event model
impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        match event.code {
            KeyCode::Enter      => Key::Enter,
            KeyCode::Esc        => Key::Esc,
            KeyCode::Char(char) => Key::Char(char),
            KeyCode::Backspace  => Key::Backspace,
            KeyCode::Up         => Key::Up,
            KeyCode::Down       => Key::Down,
            KeyCode::Left       => Key::Left,
            KeyCode::Right      => Key::Right,
            KeyCode::Tab        => Key::Tab,
            _                   => Key::Unkown,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouseKind {
    LeftClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
    Drag,
    Unkown,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mouse {
    pub kind: MouseKind,
    pub column: u16,
    pub row: u16,
}

// adapter from crossterm mouse event type to application event model
impl From<MouseEvent> for Mouse {
    fn from(event: MouseEvent) -> Self {
        let kind = match event.kind {
            MouseEventKind::Down(MouseButton::Left)     => MouseKind::LeftClick,
            MouseEventKind::Down(MouseButton::Middle)   => MouseKind::MiddleClick,
            MouseEventKind::ScrollUp                    => MouseKind::ScrollUp,
            MouseEventKind::ScrollDown                  => MouseKind::ScrollDown,
            MouseEventKind::Drag(MouseButton::Left)     => MouseKind::Drag,
            _                                           => MouseKind::Unkown,
        };

        Self {
            kind,
            column: event.column,
            row: event.row,
        }
    }
}

/*

fn process_drag_event(drag_util: DragUtility, end_pos: u16) {
    // start position is the previous position=TODO: update start position at end
    let start_pos = if let Some(start_pos) = drag_util.start_pos {
        start_pos
    } else {
        end_pos
    };

    if start_pos < end_pos {
        // send move data window left signal
    }

    if start_pos > end_pos {
        // send move data right signal
    }



    drag_util.start_pos = end_pos;


}

struct DragUtility {
    start_pos: Option<u16>,
}

fn handle_drag_event() {

}*/