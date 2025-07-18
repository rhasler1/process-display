use anyhow::{Ok, Result};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, Gauge},
};
use crate::input::*;
use crate::services::sysinfo_service::SysInfoService;
use crate::components::DrawableComponent;
use crate::models::items::memory_item::MemoryItem;
use crate::config::Config;
use super::Component;
use super::EventState;

pub struct MemoryComponent {
    config: Config,
    memory: MemoryItem,
}

impl MemoryComponent {
    pub fn new(config: Config, sysinfo: &SysInfoService) -> Self {
        let mut memory = MemoryItem::default();
        sysinfo.get_memory(&mut memory);

        Self {
            config,
            memory,
        }
    }

    pub fn update(&mut self, sysinfo: &SysInfoService) {
        sysinfo.get_memory(&mut self.memory);
    }
}

impl Component for MemoryComponent {
    fn key_event(&mut self, _key: Key) -> Result<EventState> {
        Ok(EventState::NotConsumed)
    }

    fn mouse_event(&mut self, _mouse: Mouse) -> Result<EventState> {
        Ok(EventState::NotConsumed)
    }
}

impl DrawableComponent for MemoryComponent {
    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect, focused: bool) -> Result<()> {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Ratio(1, 2),
                    Constraint::Ratio(1, 2),
                ].as_ref())
                .split(area);

        let style = if focused {
            self.config.theme_config.style_border_focused
        }
        else {
            self.config.theme_config.style_border_not_focused
        };

        // ram widget
        let ram_percent = ( self.memory.used_memory_gb() / self.memory.total_memory_gb() ) * 100_f64;
        let ram_label = "RAM Usage";
        let ram_title = format!(" {:<15} {:.2} GB / {:.2} GB ", ram_label, self.memory.used_memory_gb(), self.memory.total_memory_gb());

        let g_ram = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::TOP | Borders::RIGHT).style(style).title(ram_title))
            .gauge_style(Style::new().light_cyan().on_black().italic())
            .percent(ram_percent as u16);

        // swap widget
        let swap_percent = ( self.memory.used_swap_gb() / self.memory.total_swap_gb() ) * 100_f64;
        let swap_label = "Swap Usage";
        let swap_title = format!(" {:<15} {:.2} GB / {:.2} GB ", swap_label, self.memory.used_swap_gb(), self.memory.total_swap_gb());

        let g_swap = Gauge::default()
            .block(Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT).style(style).title(swap_title))
            .gauge_style(Style::new().light_magenta().on_black().italic())
            .percent(swap_percent as u16);

        f.render_widget(g_ram, vertical_chunks[0]);
        f.render_widget(g_swap, vertical_chunks[1]);

        Ok(())
    }
}