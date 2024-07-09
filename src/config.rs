use crossterm::event::KeyCode;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct KeyConfig {
    pub move_up: KeyCode,
    pub move_top: KeyCode,
    pub move_down: KeyCode,
    pub move_bottom: KeyCode,
    pub enter: KeyCode,
    pub tab: KeyCode,
    pub filter: KeyCode,
    pub terminate: KeyCode,
    pub tab_right: KeyCode,
    pub tab_left: KeyCode,
    pub open_help: KeyCode,
    pub exit_popup: KeyCode,
    pub sort_name_inc: KeyCode,
    pub sort_name_dec: KeyCode,
    pub sort_pid_inc: KeyCode,
    pub sort_pid_dec: KeyCode,
    pub sort_usage_inc: KeyCode,
    pub sort_usage_dec: KeyCode,
    pub follow_selection: KeyCode,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            move_up: KeyCode::Up,
            move_top: KeyCode::PageUp,
            move_down: KeyCode::Down,
            move_bottom: KeyCode::PageDown,
            enter: KeyCode::Enter,
            tab: KeyCode::Tab,
            filter: KeyCode::Char('/'),
            terminate: KeyCode::Delete,
            tab_right: KeyCode::Right,
            tab_left: KeyCode::Left,
            open_help: KeyCode::Char('?'),
            exit_popup: KeyCode::Esc,
            sort_name_inc: KeyCode::Char('n'),
            sort_name_dec: KeyCode::Char('N'),
            sort_pid_inc: KeyCode::Char('p'),
            sort_pid_dec: KeyCode::Char('P'),
            sort_usage_inc: KeyCode::Char('u'),
            sort_usage_dec: KeyCode::Char('U'),
            follow_selection: KeyCode::Char('f'),
        }
    }
}