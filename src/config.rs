use crossterm::event::KeyCode;
use serde::{Deserialize,Serialize};

#[derive(Clone,Serialize,Deserialize)]
pub struct Config {
    pub key_config: KeyConfig,
    pub theme_config: ThemeConfig,
    refresh_rate: u64,
    min_as_s: u64,
    events_per_min: u64,
    tick_rate: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_config: KeyConfig::default(),
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
    pub toggle_themes: KeyCode,
    pub process_info: KeyCode,
    pub expand: KeyCode,
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
            toggle_themes: KeyCode::Char('t'),
            process_info: KeyCode::Enter,
            expand: KeyCode::Char('e'),
        }
    }
}

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub enum ThemeVariant {
    Dark,
    Light,
}

use ratatui::prelude::{Style,Color,Modifier};
#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct ThemeConfig {
    pub theme_variant: ThemeVariant,
    pub list_header: Style,
    pub item_style: Style,      //default
    pub item_select: Style,
    pub item_select_follow: Style,
    pub component_out_of_focus: Style,
    pub component_in_focus: Style,
}

impl ThemeConfig {
    fn set_dark_theme(&mut self) {
        self.theme_variant = ThemeVariant::Dark;
        self.list_header = Style::default().fg(Color::Black).bg(Color::Gray);
        self.item_style = Style::default().fg(Color::White);
        self.item_select = Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD);
        self.item_select_follow = Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED);
        self.component_out_of_focus = Style::default().fg(Color::DarkGray);
        //self.component_in_focus = Style::default().fg(Color::LightGreen);
        self.component_in_focus = Style::default().fg(Color::White);
    }

    fn set_light_theme(&mut self) {
        self.theme_variant = ThemeVariant::Light;
        self.list_header = Style::default().fg(Color::White).bg(Color::Black);
        self.item_style = Style::default().fg(Color::Black);
        self.item_select = Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD);
        self.item_select_follow = Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED);
        self.component_out_of_focus = Style::default().fg(Color::DarkGray);
        self.component_in_focus = Style::default().fg(Color::LightGreen);
    }

    pub fn toggle_themes(&mut self) {
        if self.theme_variant == ThemeVariant::Dark {
            self.set_light_theme();
            return
        }
        self.set_dark_theme();
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            theme_variant: ThemeVariant::Dark,
            list_header: Style::default().fg(Color::Black).bg(Color::Gray),
            item_style: Style::default().fg(Color::White),
            item_select: Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD),
            item_select_follow: Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
            component_out_of_focus: Style::default().fg(Color::DarkGray),
            component_in_focus: Style::default().fg(Color::LightGreen),
            //component_in_focus: Style::default().fg(Color::White),
        }
    }
}
