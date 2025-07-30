use std::cmp::{max, min};
use anyhow::{Ok, Result};
use ratatui::{Frame, prelude::*, widgets::*};
use crate::{components::Refreshable, input::*, services::ItemProvider, states::bounded_queue_state::BoundedQueueState};
use crate::components::DrawableComponent;
use crate::models::items::memory_item::MemoryItem;
use crate::config::Config;
use super::Component;
use super::EventState;
use crate::config::*;

pub struct MemoryComponent {
    config: Config,
    queue_state: BoundedQueueState<MemoryItem>,
    data_window_time_scale: u64,
}

impl MemoryComponent {
    pub fn new<S>(config: Config, service: &S) -> Self 
    where S: ItemProvider<MemoryItem>
    {
        let capacity = ( config.max_time_span() / config.refresh_rate() ) as usize;
        let selection = None;
        let data_window_time_scale = config.default_time_span();

        let mut queue_state = BoundedQueueState::new(capacity, selection);
        let memory = service.fetch_item();
        // adding first item
        queue_state.add_item(memory);

        Self {
            config,
            queue_state,
            data_window_time_scale,
        }
    }
}

impl<S> Refreshable<S> for MemoryComponent
where
    S: ItemProvider<MemoryItem>
{
    fn refresh(&mut self, service: &S) {
        let memory_item: MemoryItem = service.fetch_item();
        self.queue_state.add_item(memory_item);
    }
}

impl Component for MemoryComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState> {
        let key_config = &self.config.key_config;
        if key == key_config.move_down {
            self.data_window_time_scale = min(self.data_window_time_scale.saturating_add(self.config.time_step()), self.config.max_time_span());
            return Ok(EventState::Consumed)
        }
        if key == key_config.move_up {
            self.data_window_time_scale = max(self.data_window_time_scale.saturating_sub(self.config.time_step()), self.config.max_time_span());
            return Ok(EventState::Consumed)
        }

        Ok(EventState::NotConsumed)
    }

    fn mouse_event(&mut self, mouse: Mouse) -> Result<EventState> {
        match mouse.kind {
            MouseKind::ScrollDown => {
                self.data_window_time_scale = min(self.data_window_time_scale.saturating_add(self.config.time_step()), self.config.max_time_span());
                return Ok(EventState::Consumed)
            }
            MouseKind::ScrollUp => { 
                self.data_window_time_scale = max(self.data_window_time_scale.saturating_sub(self.config.time_step()), self.config.max_time_span());
                return Ok(EventState::Consumed)
            }
            _ => {}
        }

        Ok(EventState::NotConsumed)
    }
}

impl DrawableComponent for MemoryComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()> {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(0),
            ]).split(area);

        let refresh_rate = self.config.refresh_rate();              // default = 2,000 ms
        let time_scale = self.data_window_time_scale;               // default = 60,000 ms
        let data_window = (time_scale / refresh_rate) as usize;     // default = 30
        let max_idx = data_window.saturating_sub(1);                //


        // building data sets
        let ram_percent_usage_data: Vec<(f64, f64)> = self.queue_state
            .iter()
            .rev()
            .take(data_window)
            .enumerate()
            .map(|(idx, memory_item)| {
                (
                    max_idx.saturating_sub(idx) as f64, 
                    memory_item.percent_memory_usage(),
                )
            })
            .collect();

        let swap_percent_usage_data: Vec<(f64, f64)> = self.queue_state
            .iter()
            .rev()
            .take(data_window)
            .enumerate()
            .map(|(idx, memory_item)| {
                (
                    max_idx.saturating_sub(idx) as f64, 
                    memory_item.percent_swap_usage(),
                )
            })
            .collect();

        let datasets = vec![
            Dataset::default()
                .data(&ram_percent_usage_data)
                .graph_type(GraphType::Line)
                .marker(symbols::Marker::Braille)
                .style(Style::new().light_red()),
            
            Dataset::default()
                .data(&swap_percent_usage_data)
                .graph_type(GraphType::Line)
                .marker(symbols::Marker::Braille)
                .style(Style::new().light_magenta()),
        ];

        // set block style
        let block_style = if focused {
            self.config.theme_config.style_border_focused
        }
        else {
            self.config.theme_config.style_border_not_focused
        };

        // building chart
        let x_axis = Axis::default()
            .bounds([0.0, max_idx as f64])
            .labels(vec![Span::raw(format!("{}s", ms_to_s(time_scale))), Span::raw("now")])
            .labels_alignment(Alignment::Right);

        let y_axis = Axis::default()
            .bounds([0.0, 100.0])
            .labels(vec![
                Span::raw("0%"),
                Span::raw("50"),
                Span::raw("100"),
            ])
            .labels_alignment(Alignment::Right);
        
        let chart_title = format!(" Memory ");
        let chart = Chart::new(datasets)
            .block(Block::default()
                .borders(Borders::LEFT|Borders::BOTTOM|Borders::RIGHT)
                .style(block_style)
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

        f.render_widget(chart, vertical_chunks[1]);


        // building legend
        let memory_item = if let Some(item) = self.queue_state.back() {
            item
        }
        else {
            &MemoryItem::default()
        };
        let ram_legend = format!(" RAM :: {:.0}/{:.0}GB :: {:.0}%",
            memory_item.used_memory_gb(),
            memory_item.total_memory_gb(),
            memory_item.percent_memory_usage(),
        );
        let swap_legend = format!(" SWAP :: {:.0}/{:.0}GB :: {:.0}%",
            memory_item.used_swap_gb(),
            memory_item.total_swap_gb(),
            memory_item.percent_swap_usage(),
        );

        let legend = Paragraph::new(
            Line::from(vec![
                Span::styled(ram_legend, Style::default().fg(Color::LightRed)),
                Span::raw("  "),
                Span::styled(swap_legend, Style::default().fg(Color::LightMagenta)),
            ])
            .right_aligned())
            .block(Block::new()
                .borders(Borders::LEFT|Borders::TOP|Borders::RIGHT)
                .style(block_style)
                .title(chart_title)
            );
        
        f.render_widget(legend, vertical_chunks[0]);

        Ok(())
    }
}