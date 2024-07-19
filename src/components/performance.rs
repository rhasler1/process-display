use std::io;
use crossterm::event::KeyEvent;
use ratatui::widgets::Dataset;
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use super::EventState;
use super::DrawableComponent;
use crate::performance_structs::cpu_perf_info::CpuInfo;
use crate::performance_structs::perf_item::CpuItem;
use crate::config::KeyConfig;
use crate::components::Component;

#[derive(Default, Clone)]
pub struct PerformanceComponent {
    cpu_info: CpuInfo,
    _key_config: KeyConfig,
}

impl PerformanceComponent {
    pub fn new(key_config: KeyConfig, max_size: usize) -> Self {
        Self {
            _key_config: key_config,
            cpu_info: CpuInfo::new(max_size),
        }
    }

    pub fn update(&mut self, item: &CpuItem) -> io::Result<()> {
        self.cpu_info.add_item(item)?;
        Ok(())
    }
}

impl Component for PerformanceComponent {
    fn event(&mut self, _key: KeyEvent) -> io::Result<EventState> {
        //todo
        Ok(EventState::NotConsumed)
        
    }
}

impl DrawableComponent for PerformanceComponent {
    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect, _focused: bool) -> io::Result<()> {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // filter chunk
                Constraint::Min(1), // graph chunk
                Constraint::Length(6), // cpu info
            ].as_ref())
            .split(area);

        let split_graph_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30), // more tabs
                    Constraint::Percentage(70), // total cpu usage over time graph
                ].as_ref())
                .split(vertical_chunks[1]);

        let refresh_rate = 5;
        let data_points = self.cpu_info.cpu_items.cpu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                ((i*refresh_rate) as f64, item.total_usage() as f64)
            })
            .collect::<Vec<_>>();

        let data_set = vec![
            Dataset::default()
                .name("Total Cpu Usage (%)")
                .marker(Marker::Dot)
                .graph_type(GraphType::Line)
                .style(Style::default().cyan())
                .data(&data_points)
        ];

        let x_axis = Axis::default()
            .title("time")
            .style(Style::default().white())
            .bounds([0.0, (self.cpu_info.max_size() * refresh_rate) as f64])
            .labels(vec![0.to_string().into(), (self.cpu_info.max_size() * refresh_rate).to_string().into()]);

        let y_axis = Axis::default()
            .title("Usage")
            .style(Style::default().white())
            .bounds([0.0, 100.0])
            .labels(vec![0.to_string().into(), 100.to_string().into()]);

        let chart = Chart::new(data_set)
            .block(Block::default().title("Total Cpu Usage (%)"))
            .x_axis(x_axis)
            .y_axis(y_axis);

        f.render_widget(chart, split_graph_area[1]);

        Ok(())
    }
}