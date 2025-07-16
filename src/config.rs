use crossterm::event::{KeyCode, MouseButton, MouseEventKind};
use serde::{Deserialize,Serialize};

#[derive(Clone,Serialize,Deserialize)]
pub struct Config {
    pub key_config: KeyConfig,
    pub mouse_config: MouseConfig,
    pub theme_config: ThemeConfig,
    refresh_rate: u64,
    min_as_s: u64,
    events_per_min: u64,
    tick_rate: u64,
}

//times are in ms
impl Default for Config {
    fn default() -> Self {
        Self {
            key_config: KeyConfig::default(),
            mouse_config: MouseConfig::default(),
            theme_config: ThemeConfig::default(),
            refresh_rate: 2000,
            min_as_s: 60000/ 1000,
            events_per_min: 60000 / 2000,
            tick_rate: 250,
        }
    }
}

impl Config {
    pub fn refresh_rate(&self) -> u64 {
        self.refresh_rate
    }

    pub fn tick_rate(&self) -> u64 {
        self.tick_rate
    }

    pub fn min_as_s(&self) -> u64 {
        self.min_as_s
    }
 
    pub fn events_per_min(&self) -> u64 {
        self.events_per_min
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
    pub exit: KeyCode,
    pub sort_name_inc: KeyCode,
    pub sort_name_dec: KeyCode,
    pub sort_pid_inc: KeyCode,
    pub sort_pid_dec: KeyCode,
    pub sort_cpu_usage_inc: KeyCode,
    pub sort_cpu_usage_dec: KeyCode,
    pub sort_memory_usage_inc: KeyCode,
    pub sort_memory_usage_dec: KeyCode,
    pub follow_selection: KeyCode,
    pub toggle_themes: KeyCode,
    pub process_info: KeyCode,
    pub expand: KeyCode,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            move_up: KeyCode::Up,
            move_top: KeyCode::Char('W'),
            move_down: KeyCode::Down,
            move_bottom: KeyCode::Char('S'),
            enter: KeyCode::Enter,
            tab: KeyCode::Tab,
            filter: KeyCode::Char('/'),
            terminate: KeyCode::Char('T'),
            tab_right: KeyCode::Right,
            tab_left: KeyCode::Left,
            open_help: KeyCode::Char('?'),
            exit: KeyCode::Esc,
            sort_name_inc: KeyCode::Char('n'),
            sort_name_dec: KeyCode::Char('N'),
            sort_pid_inc: KeyCode::Char('p'),
            sort_pid_dec: KeyCode::Char('P'),
            sort_cpu_usage_inc: KeyCode::Char('c'),
            sort_cpu_usage_dec: KeyCode::Char('C'),
            sort_memory_usage_inc: KeyCode::Char('m'),
            sort_memory_usage_dec: KeyCode::Char('M'),
            follow_selection: KeyCode::Char('f'),
            toggle_themes: KeyCode::Char('t'),
            process_info: KeyCode::Enter,
            expand: KeyCode::Char('e'),
        }
    }
}

#[derive(Clone,Serialize,Deserialize)]
pub struct MouseConfig {
    pub left_click: MouseButton,
    pub right_click: MouseButton,
    pub scroll_up: MouseEventKind,
    pub scroll_down: MouseEventKind,
}

impl Default for MouseConfig {
    fn default() -> Self {
        Self {
            left_click: MouseButton::Left,
            right_click: MouseButton::Right,
            scroll_up: MouseEventKind::ScrollUp,
            scroll_down: MouseEventKind::ScrollDown,
        }
    }
}


use ratatui::prelude::{Color, Modifier, Style};
#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct ThemeConfig {
    pub style_border_focused: Style,
    pub style_border_not_focused: Style,
    pub style_item_focused: Style,
    pub style_item_not_focused: Style,
    pub style_item_selected: Style,
    pub style_item_selected_not_focused: Style,
    pub style_item_selected_followed: Style,
    pub style_item_selected_followed_not_focused: Style,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            style_border_focused: Style::default().fg(Color::LightGreen),
            style_border_not_focused: Style::default().fg(Color::DarkGray),
            style_item_focused: Style::default().fg(Color::White),
            style_item_not_focused: Style::default().fg(Color::DarkGray),
            style_item_selected: Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD),
            style_item_selected_not_focused: Style::default().bg(Color::Gray).add_modifier(Modifier::BOLD),
            style_item_selected_followed: Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
            style_item_selected_followed_not_focused: Style::default().bg(Color::Gray).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
        }
    }
}
