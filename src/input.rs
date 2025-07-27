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
            _                                           => MouseKind::Unkown,
        };

        Self {
            kind,
            column: event.column,
            row: event.row,
        }
    }
}