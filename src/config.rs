use std::ops::Div;
use serde::{Deserialize,Serialize};

#[derive(Clone)]
pub struct Config {
    pub key_config: KeyConfig,
    pub mouse_config: MouseConfig,
    pub theme_config: ThemeConfig,
    refresh_rate: u64,
    max_time_span: u64,
    default_time_span: u64,
    time_step: u64,
    tick_rate: u64,
}

impl Default for Config {
    fn default() -> Self {
        let refresh_rate = 2000;                    // ms (2 seconds)      
        let max_time_span = 60000;                 // ms (10 minutes)
        let default_time_span = 16000;              // ms (60 seconds)
        let time_step = 2000;                      // ms (15 seconds)
        let tick_rate = 250;                        // ms


        Self {
            key_config: KeyConfig::default(),
            mouse_config: MouseConfig::default(),
            theme_config: ThemeConfig::default(),
            refresh_rate,
            max_time_span,
            default_time_span,
            time_step,
            tick_rate,
        }
    }
}

impl Config {
    pub fn refresh_rate(&self) -> u64 {
        self.refresh_rate
    }

    pub fn max_time_span(&self) -> u64 {
        self.max_time_span
    }

    pub fn default_time_span(&self) -> u64 {
        self.default_time_span
    }

    pub fn time_step(&self) -> u64 {
        self.time_step
    }

    pub fn tick_rate(&self) -> u64 {
        self.tick_rate
    } 
}

pub fn ms_to_s(data_ms: u64) -> u64 {
    data_ms.div(1000)
}

#[derive(Clone)]
pub struct KeyConfig {
    pub move_up: Key,
    pub move_top: Key,
    pub move_down: Key,
    pub move_bottom: Key,
    pub move_left: Key,
    pub move_right: Key,
    pub enter: Key,
    pub tab: Key,
    pub filter: Key,
    pub terminate: Key,
    pub help: Key,
    pub exit: Key,
    pub sort_name_toggle: Key,
    pub sort_pid_toggle: Key,
    pub sort_cpu_toggle: Key,
    pub sort_memory_toggle: Key,
    pub follow_selection: Key,
    pub expand: Key,
    pub freeze: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            move_up: Key::Up,
            move_top: Key::Char('W'),
            move_down: Key::Down,
            move_bottom: Key::Char('S'),
            move_left: Key::Left,
            move_right: Key::Right,
            enter: Key::Enter,
            tab: Key::Tab,
            filter: Key::Char('/'),
            terminate: Key::Char('T'),
            help: Key::Char('?'),
            exit: Key::Esc,
            sort_name_toggle: Key::Char('n'),
            sort_pid_toggle: Key::Char('p'),
            sort_cpu_toggle: Key::Char('c'),
            sort_memory_toggle: Key::Char('m'),
            follow_selection: Key::Char('f'),
            expand: Key::Char('e'),
            freeze: Key::Char('f'),
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

use ratatui::{prelude::{Color, Style}, style::Modifier};
use crate::input::{Key, MouseKind};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct ThemeConfig {
    pub style_border_focused: Style,
    pub style_border_not_focused: Style,
    pub style_item_focused: Style,
    pub style_item_not_focused: Style,
    pub style_item_selected: Style,
    pub style_item_selected_not_focused: Style,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            style_border_focused: Style::default().fg(Color::LightGreen),
            style_border_not_focused: Style::default().fg(Color::DarkGray),

            style_item_focused: Style::default().fg(Color::White),
            style_item_not_focused: Style::default().fg(Color::DarkGray),

            style_item_selected: Style::default().fg(Color::LightBlue),
            style_item_selected_not_focused: Style::default().fg(Color::Gray),
        }
    }
}