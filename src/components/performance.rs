use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::widgets::Dataset;
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use performance_queue::{CpuItem, MemoryItem, PerformanceQueue};
use super::EventState;
use super::DrawableComponent;
use super::vertical_tabs::VerticalTab;
use crate::config::Config;
use crate::components::Component;
use crate::components::vertical_tabs::VerticalTabComponent;

//#[derive(Default)]
pub struct PerformanceComponent {
    cpu_info: PerformanceQueue<CpuItem>,
    memory_info: PerformanceQueue<MemoryItem>,
    vertical_tabs: VerticalTabComponent,
    pub config: Config,
}

impl PerformanceComponent {
    pub fn new(config: Config, max_size: usize) -> Self {
        Self {
            cpu_info: PerformanceQueue::new(max_size),
            memory_info: PerformanceQueue::new(max_size),
            vertical_tabs: VerticalTabComponent::default(),
            config,
        }
    }

    pub fn update(&mut self, cpu_item: &CpuItem, memory_item: &MemoryItem) -> Result<()> {
        self.cpu_info.add_item(cpu_item)?;
        self.memory_info.add_item(memory_item)?;
        Ok(())
    }

    fn draw_memory_graph(&self, f: &mut Frame, area: Rect) -> Result<()> {
        let refresh_rate = (self.config.refresh_rate() / 1000) as usize; // Converting ms to s.
        let data_points = self.memory_info.performance_items
            .iter()
            .enumerate()
            .map(|(i, item)| ((i * refresh_rate) as f64, item.used_memory_gb() as f64))
            .collect::<Vec<_>>();

        let y_axis_title = String::from("Used Memory (GB)");
        let y_bounds = [0.0, self.memory_info.back().unwrap().total_memory_gb() as f64];
        draw_graph(f, area, refresh_rate, self.memory_info.max_size(), data_points, y_axis_title, y_bounds)?;
        Ok(())
    }

    fn draw_cpu_graph(&self, f: &mut Frame, area: Rect) -> Result<()> {
        let refresh_rate = (self.config.refresh_rate() / 1000) as usize; // Converting ms to s.
        let data_points = self.cpu_info.performance_items
            .iter()
            .enumerate()
            .map(|(i, item)| ((i * refresh_rate) as f64, item.global_usage() as f64))
            .collect::<Vec<_>>();

        let y_axis_title = String::from("Global CPU Usage (%)");
        let y_bounds = [0.0, 100.0];
        draw_graph(f, area, refresh_rate, self.cpu_info.max_size(), data_points, y_axis_title, y_bounds)?;
        Ok(())
    }

    fn draw_cpu_item(&self, f: &mut Frame, area: Rect) -> Result<()> {
        //TODO
        if let Some(item) = self.cpu_info.back() {
            let info = vec![
                Line::from(vec![
                    Span::raw("Global CPU Usage: "),
                    Span::raw(item.global_usage().to_string()),
                    Span::raw("%"),
                ])
                .style(Color::White),
                Line::from(vec![
                    Span::raw("CPU Brand: "),
                    Span::raw(item.brand().to_string()),
                ])
                .style(Color::White),
                Line::from(vec![
                    Span::raw("Number of Cores: "),
                    Span::raw(item.num_cores().unwrap_or_default().to_string()),
                ])
                .style(Color::White),
                Line::from(vec![
                    Span::raw("Frequency: "),
                    Span::raw(item.frequency().to_string()),
                    Span::raw(" MHz"),
                ])
                .style(Color::White),
            ];

            let widget = Paragraph::new(info)
                .block(Block::default().title("Cpu Info").borders(Borders::ALL))
                .style(Style::default().fg(Color::DarkGray));

            f.render_widget(widget, area);
        }
        Ok(())
    }

    fn draw_memory_item(&self, f: &mut Frame, area: Rect) -> Result<()> {
        if let Some(item) = self.memory_info.back() {
            let info = vec![
                Line::from(vec![
                    Span::raw("Total RAM: "),
                    Span::raw(item.total_memory_gb().to_string()),
                    Span::raw(" GB"),
                ])
                .style(Color::White),
                Line::from(vec![
                    Span::raw("Used RAM: "),
                    Span::raw(item.used_memory_gb().to_string()),
                    Span::raw(" GB"),
                ])
                .style(Color::White),
                Line::from(vec![
                    Span::raw("Free RAM: "),
                    Span::raw(item.free_memory_gb().to_string()),
                    Span::raw(" GB"),
                ])
                .style(Color::White),
                Line::from(vec![
                    Span::raw("Available RAM: "),
                    Span::raw(item.available_memory_gb().to_string()),
                    Span::raw(" GB"),
                ])
                .style(Color::White),
            ];

            let widget = Paragraph::new(info)
                .block(Block::default().title("Memory Info").borders(Borders::ALL))
                .style(Style::default().fg(Color::DarkGray));

            f.render_widget(widget, area);
        }
        Ok(())
    }
}

impl Component for PerformanceComponent {
    fn event(&mut self, key: KeyEvent) -> Result<EventState> {
        //todo
        self.vertical_tabs.event(key)?;
        Ok(EventState::NotConsumed)  
    }
}

impl DrawableComponent for PerformanceComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> Result<()> {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // filter chunk
                Constraint::Min(1), // graph chunk
                Constraint::Length(6), // cpu info
            ].as_ref())
            .split(area);

        let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50), // more tabs
                    Constraint::Percentage(50), // total cpu usage over time graph
                ].as_ref())
                .split(vertical_chunks[2]);
        
        if matches!(self.vertical_tabs.selected_vert_tab, VerticalTab::Cpu) {
            self.draw_cpu_graph(f, vertical_chunks[1])?;
            self.draw_cpu_item(f, horizontal_chunks[1])?;
        }
        else if matches!(self.vertical_tabs.selected_vert_tab, VerticalTab::Memory) {
            self.draw_memory_graph(f, vertical_chunks[1])?;
            self.draw_memory_item(f, horizontal_chunks[1])?;
        }
        //else if matches!(self.vertical_tabs.selected_vert_tab, VerticalTab::Network) {
            //self.draw_network_graph(f, vertical_chunks[1])?;
        //}
        self.vertical_tabs.draw(f, horizontal_chunks[0], false)?;

        Ok(())
    }
}

fn draw_graph(
    f: &mut Frame,
    area: Rect,
    refresh_rate: usize,
    max_size: usize,
    data_points: Vec<(f64, f64)>,
    y_axis_title: String,
    y_bounds: [f64; 2],
) -> Result<()> {
    let data_set = vec![
        Dataset::default()
            .marker(Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().cyan())
            .data(&data_points)
    ];

    let x_axis = Axis::default()
        .title("Time (s)")
        .style(Style::default().white())
        .bounds([0.0, ((max_size - 1) * refresh_rate as usize) as f64])
        .labels(vec![
            ((max_size - 1) * refresh_rate as usize).to_string().into(),
            0.to_string().into(),
        ]);

    let y_axis = Axis::default()
        .title(y_axis_title)
        .style(Style::default().white())
        .bounds(y_bounds)
        .labels(vec![y_bounds[0].to_string().into(), y_bounds[1].to_string().into()]);

    let chart = Chart::new(data_set)
        .block(Block::default())
        .x_axis(x_axis)
        .y_axis(y_axis);

    f.render_widget(chart, area);

    Ok(())
}