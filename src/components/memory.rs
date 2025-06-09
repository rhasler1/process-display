use std::fs::File;

// RAM component
use anyhow::Ok;
use anyhow::Result;
use bounded_queue::BoundedQueue;
use bounded_queue::MemoryItem;
use crate::components::DrawableComponent;
use crate::config::Config;
use super::Component;
use super::EventState;
use ratatui::{
    layout::{Layout, Direction, Constraint},
    style::{Style, Stylize},
    widgets::{Block, Gauge},
};

//TODO: This does not need to use a bounded queue--only displaying fields of most recent item with gauges.
pub struct MemoryComponent {
    config: Config,
    memories: BoundedQueue<MemoryItem>,
}

impl MemoryComponent {
    pub fn new(config: Config) -> Self {
        let memories: BoundedQueue<MemoryItem> = BoundedQueue::new(config.events_per_min() as usize);

        Self {
            config,
            memories,
        }
    }

    pub fn update(&mut self, memory_item: MemoryItem) {
        self.memories.add_item(memory_item);
    }
}

impl Component for MemoryComponent {
    fn event(&mut self, _key: crossterm::event::KeyEvent) -> Result<EventState> {
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
        let ram_percent = if let Some(item) = self.memories.back() {
            ( item.used_memory_gb() / item.total_memory_gb() ) * 100_f64
        }
        else {
            0_f64
        };

        let ram_label = "RAM Usage";
        let ram_title = if let Some(item) = self.memories.back() {
            format!(" {:<15} {:.2} GB / {:.2} GB ", ram_label, item.used_memory_gb(), item.total_memory_gb())
        }
        else {
            format!(" {:<15} ", ram_label)
        };

        let g_ram = Gauge::default()
            .block(Block::bordered().style(style).title(ram_title))
            .gauge_style(Style::new().red().on_black().italic())
            .percent(ram_percent as u16);

        // swap widget
        let swap_percent = if let Some(item) = self.memories.back() {
            ( item.used_swap_gb() / item.total_swap_gb() ) * 100_f64
        }
        else {
            0_f64
        };

        let swap_label = "Swap Usage";
        let swap_title = if let Some(item) = self.memories.back() {
            format!(" {:<15} {:.2} GB / {:.2} GB ", swap_label, item.used_swap_gb(), item.total_swap_gb())
        }
        else {
            format!(" {:<15} ", swap_label)
        };

        let g_swap = Gauge::default()
            .block(Block::bordered().style(style).title(swap_title))
            .gauge_style(Style::new().magenta().on_black().italic())
            .percent(swap_percent as u16);

        f.render_widget(g_ram, vertical_chunks[0]);
        f.render_widget(g_swap, vertical_chunks[1]);

        Ok(())
    }
}