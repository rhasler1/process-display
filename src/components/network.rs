
use std::cmp::{max, min};

use anyhow::{Ok, Result};
use ratatui::{Frame, prelude::*, widgets::*};

use crate::components::Refreshable;
use crate::input::MouseKind;
use crate::services::ItemProvider;
use crate::states::bounded_queue_state::BoundedQueueState;
use crate::models::items::network_item::NetworkItem;
use crate::models::items::*;
use crate::config::Config;
use crate::components::*;
use crate::config::*;

pub struct NetworkComponent {
    config: Config,
    queue_state: BoundedQueueState<NetworkItem>,
    data_window_time_scale: u64,
}

impl NetworkComponent {
    pub fn new<S>(config: Config, service: &S) -> Self
    where S: ItemProvider<NetworkItem>
    {
        let capacity = ( config.max_time_scale() / config.refresh_rate() ) as usize;
        let selection = None;
        let refresh_bool = true;
        let data_window_time_scale = config.min_time_scale();

        let mut queue_state = BoundedQueueState::new(capacity, selection, refresh_bool);
        let network = service.fetch_item();

        queue_state.add_item(network);

        Self {
            config,
            queue_state,
            data_window_time_scale,
        }
    }
}

impl<S> Refreshable<S> for NetworkComponent
where
    S: ItemProvider<NetworkItem>
{
    fn refresh(&mut self, service: &S) {
        let network_item: NetworkItem = service.fetch_item();
        self.queue_state.add_item(network_item);
    }
}

impl Component for NetworkComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState> {
        let key_config = &self.config.key_config;
        if key == key_config.move_down {
            self.data_window_time_scale = min(self.data_window_time_scale.saturating_add(self.config.time_inc()), self.config.max_time_scale());
            return Ok(EventState::Consumed)
        }
        if key == key_config.move_up {
            self.data_window_time_scale = max(self.data_window_time_scale.saturating_sub(self.config.time_inc()), self.config.min_time_scale());
            return Ok(EventState::Consumed)
        }

        Ok(EventState::NotConsumed)
    }

    fn mouse_event(&mut self, mouse: Mouse) -> Result<EventState> {
        match mouse.kind {
            MouseKind::ScrollDown => {
                self.data_window_time_scale = min(self.data_window_time_scale.saturating_add(self.config.time_inc()), self.config.max_time_scale());
                return Ok(EventState::Consumed)
            }
            MouseKind::ScrollUp => { 
                self.data_window_time_scale = max(self.data_window_time_scale.saturating_sub(self.config.time_inc()), self.config.min_time_scale());
                return Ok(EventState::Consumed)
            }
            _ => {}
        }

        Ok(EventState::NotConsumed)
    }
}

impl DrawableComponent for NetworkComponent {
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
        let y_max_scale_factor = 1.5;                               // used to scale y-upper bound

        let tx_data: Vec<(f64, f64)> = self.queue_state
                .iter()
                .rev()
                .take(data_window)
                .enumerate()
                .map(|(idx, network_item)| {
                    (
                        max_idx.saturating_sub(idx) as f64,
                        byte_to_kb(network_item.tx()) as f64,
                    )
                })
                .collect();

        let rx_data: Vec<(f64, f64)> = self.queue_state
                .iter()
                .rev()
                .take(data_window)
                .enumerate()
                .map(|(idx, network_item)| {
                    (
                        max_idx.saturating_sub(idx) as f64,
                        byte_to_kb(network_item.rx()) as f64,
                    )
                })
                .collect();

        // getting upper bound on y
        let max_y = tx_data
            .iter()
            .chain(rx_data.iter())
            .map(|tuple| tuple.1)
            .fold(0.0, f64::max);

        // scaling
        let max_y = max_y * y_max_scale_factor;
        
        let datasets = vec![
            Dataset::default()
                .data(&tx_data)
                .graph_type(GraphType::Line)
                .marker(Marker::Braille)
                .style(Style::new().light_blue()),
            
            Dataset::default()
                .data(&rx_data)
                .graph_type(GraphType::Line)
                .marker(Marker::Braille)
                .style(Style::new().light_yellow())
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
            .bounds([0.0, max_y])
            .labels(vec![
                Span::raw("0KB"),
                Span::raw(format!("{}", max_y)),
            ])
            .labels_alignment(Alignment::Right);

        let chart_title = " Network ";
        let chart = Chart::new(datasets)
            .block(Block::default()
                .borders(Borders::LEFT|Borders::BOTTOM|Borders::RIGHT)
                .style(block_style)
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

        f.render_widget(chart, vertical_chunks[1]);

        let network_item = if let Some(item) = self.queue_state.back() {
            item
        }
        else {
            &NetworkItem::default()
        };
        
        let tx_per_s = network_item.tx() / ms_to_s(refresh_rate);
        let rx_per_s = network_item.rx() / ms_to_s(refresh_rate);
        let tx_legend = format!("TX/s {}KB :: TOTAL TX {}MB", byte_to_kb(tx_per_s), byte_to_mb(network_item.total_tx()));
        let rx_legend = format!("RX/s {}KB :: TOTAL RX {}MB", byte_to_kb(rx_per_s), byte_to_mb(network_item.total_rx()));

        let legend = Paragraph::new(
            Line::from(vec![
                Span::styled(tx_legend, Style::default().fg(Color::LightBlue)),
                Span::raw("  "),
                Span::styled(rx_legend, Style::default().fg(Color::LightYellow)),
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