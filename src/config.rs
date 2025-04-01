use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub key_config: KeyConfig,
    refresh_rate: u64,
    tick_rate: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_config: KeyConfig::default(),
            refresh_rate: 5000,
            tick_rate: 250,
        }
    }
}

impl Config {
    pub fn refresh_rate(&self) -> u64 {
        return self.refresh_rate.clone()
    }

    pub fn tick_rate(&self) -> u64 {
        return self.tick_rate.clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub sort_cpu_usage_inc: KeyCode,
    pub sort_cpu_usage_dec: KeyCode,
    pub sort_memory_usage_inc: KeyCode,
    pub sort_memory_usage_dec: KeyCode,
    pub follow_selection: KeyCode,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            move_up: KeyCode::Up,
            //move_up: KeyCode::Char('w'),
            move_top: KeyCode::Char('W'),
            move_down: KeyCode::Down,
            //move_down: KeyCode::Char('s'),
            move_bottom: KeyCode::Char('S'),
            enter: KeyCode::Enter,
            tab: KeyCode::Tab,
            filter: KeyCode::Char('/'),
            terminate: KeyCode::Char('T'),
            tab_right: KeyCode::Right,
            //tab_right: KeyCode::Char('d'),
            tab_left: KeyCode::Left,
            //tab_left: KeyCode::Char('a'),
            open_help: KeyCode::Char('?'),
            exit_popup: KeyCode::Esc,
            sort_name_inc: KeyCode::Char('n'),
            sort_name_dec: KeyCode::Char('N'),
            sort_pid_inc: KeyCode::Char('p'),
            sort_pid_dec: KeyCode::Char('P'),
            sort_cpu_usage_inc: KeyCode::Char('c'),
            sort_cpu_usage_dec: KeyCode::Char('C'),
            sort_memory_usage_inc: KeyCode::Char('m'),
            sort_memory_usage_dec: KeyCode::Char('M'),
            follow_selection: KeyCode::Char('f'),
        }
    }
}