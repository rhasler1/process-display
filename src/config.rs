use crossterm::event::KeyCode;

pub struct Config {
    pub key_config: KeyConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_config: KeyConfig::default(),
        }
    }
}

pub struct KeyConfig {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub enter: KeyCode,
    pub tab: KeyCode,
    pub filter: KeyCode,
    pub quit: KeyCode,
    pub terminate: KeyCode,
    pub suspend: KeyCode,
    pub resume: KeyCode,
    pub reset: KeyCode,
    pub tab_right: KeyCode,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            move_up: KeyCode::Up,
            move_down: KeyCode::Down,
            enter: KeyCode::Enter,
            tab: KeyCode::Tab,
            filter: KeyCode::Char('/'),
            quit: KeyCode::Char('q'),
            terminate: KeyCode::Delete,
            suspend: KeyCode::Char('s'),
            resume: KeyCode::Char('r'),
            reset: KeyCode::Esc,
            tab_right: KeyCode::Right,
        }
    }
}