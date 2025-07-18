use serde::{Deserialize,Serialize};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct KeyConfig {
    pub move_up: Key,
    pub move_top: Key,
    pub move_down: Key,
    pub move_bottom: Key,
    pub enter: Key,
    pub tab: Key,
    pub filter: Key,
    pub terminate: Key,
    pub open_help: Key,
    pub exit: Key,
    pub sort_name_inc: Key,
    pub sort_name_dec: Key,
    pub sort_pid_inc: Key,
    pub sort_pid_dec: Key,
    pub sort_cpu_usage_inc: Key,
    pub sort_cpu_usage_dec: Key,
    pub sort_memory_usage_inc: Key,
    pub sort_memory_usage_dec: Key,
    pub follow_selection: Key,
    pub expand: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            move_up: Key::Up,
            move_top: Key::Char('W'),
            move_down: Key::Down,
            move_bottom: Key::Char('S'),
            enter: Key::Enter,
            tab: Key::Tab,
            filter: Key::Char('/'),
            terminate: Key::Char('T'),
            open_help: Key::Char('?'),
            exit: Key::Esc,
            sort_name_inc: Key::Char('n'),
            sort_name_dec: Key::Char('N'),
            sort_pid_inc: Key::Char('p'),
            sort_pid_dec: Key::Char('P'),
            sort_cpu_usage_inc: Key::Char('c'),
            sort_cpu_usage_dec: Key::Char('C'),
            sort_memory_usage_inc: Key::Char('m'),
            sort_memory_usage_dec: Key::Char('M'),
            follow_selection: Key::Char('f'),
            expand: Key::Char('e'),
        }
    }
}

#[derive(Clone)]
pub struct MouseConfig {
    pub left_click: MouseKind,
    pub middle_click: MouseKind,
    pub scroll_up: MouseKind,
    pub scroll_down: MouseKind,
}

impl Default for MouseConfig {
    fn default() -> Self {
        Self {
            left_click: MouseKind::LeftClick,
            middle_click: MouseKind::MiddleClick,
            scroll_up: MouseKind::ScrollUp,
            scroll_down: MouseKind::ScrollDown,
        }
    }
}

use ratatui::{prelude::{Color, Modifier, Style}, style::Stylize};

use crate::input::{Key, MouseKind};
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
            style_border_focused: Style::default().fg(Color::LightGreen).bold(),
            style_border_not_focused: Style::default().fg(Color::DarkGray),

            style_item_focused: Style::default().fg(Color::White),
            style_item_not_focused: Style::default().fg(Color::Gray),

            style_item_selected: Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            style_item_selected_not_focused: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),

            style_item_selected_followed: Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
            style_item_selected_followed_not_focused: Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
        }
    }
}