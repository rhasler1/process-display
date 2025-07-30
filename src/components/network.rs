use anyhow::{Ok, Result};
use ratatui::{Frame, prelude::*, widgets::*};
use crate::components::utils::data_window::DataWindow;
use crate::components::Refreshable;
use crate::input::MouseKind;
use crate::services::ItemProvider;
use crate::states::bounded_queue_state::BoundedQueueState;
use crate::models::items::network_item::NetworkItem;
use crate::models::items::*;
use crate::config::Config;
use crate::components::*;
use crate::config::*;

const Y_UPPER_BOUND_SCALE_FACTOR: f64 = 1.5;

pub struct NetworkComponent {
    config: Config,
    queue_state: BoundedQueueState<NetworkItem>,
    data_window: DataWindow,
}

impl NetworkComponent {
    pub fn new<S>(config: Config, service: &S) -> Self
    where S: ItemProvider<NetworkItem>
    {
        let data_window_capacity = ( config.max_time_span() / config.refresh_rate() ) as usize;
        let data_window_length = ( config.default_time_span() / config.refresh_rate() ) as usize;
        let data_window_offset = 0;
        let data_window = DataWindow::new(data_window_offset, data_window_length, data_window_capacity).unwrap();

        let selection = None;
        let mut queue_state = BoundedQueueState::new(data_window_capacity, selection);

        let network = service.fetch_item();
        queue_state.add_item(network);

        Self {
            config,
            queue_state,
            data_window,
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
        let time_step = self.config.time_step();
        let refresh_rate = self.config.refresh_rate();
        let idx_step  = time_step / refresh_rate;

        if key == key_config.move_down {
            self.data_window.zoom_out(idx_step as usize);

            return Ok(EventState::Consumed)
        }
        if key == key_config.move_up {
            self.data_window.zoom_in(idx_step as usize);

            return Ok(EventState::Consumed)
        }
        if key == key_config.move_left {
            self.data_window.pan_positive(idx_step as usize);

            return Ok(EventState::Consumed)
        }
        if key == key_config.move_right {
            self.data_window.pan_negative(idx_step as usize);

            return Ok(EventState::Consumed)
        }

        Ok(EventState::NotConsumed)
    }

    fn mouse_event(&mut self, mouse: Mouse) -> Result<EventState> {
        let time_step = self.config.time_step();
        let refresh_rate = self.config.refresh_rate();
        let idx_step  = time_step / refresh_rate;

        match mouse.kind {
            MouseKind::ScrollDown => {
                self.data_window.zoom_out(idx_step as usize);
                return Ok(EventState::Consumed)
            }
            MouseKind::ScrollUp => { 
                self.data_window.zoom_in(idx_step as usize);
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
        
        let refresh_rate = self.config.refresh_rate();
        let data_window_length = self.data_window.window_length();
        let data_window_offset = self.data_window.window_offset();
        let y_upper_bound_scale_factor = Y_UPPER_BOUND_SCALE_FACTOR;

        let visible_buffer_length = self.queue_state
            .iter()
            .rev()
            .skip(data_window_offset)
            .take(data_window_length)
            .count();

        // mapping the visible data buffer to x,y-coordinates
        let tx_visible_data: Vec<(f64, f64)> = self.queue_state
                .iter()
                .rev()
                .skip(data_window_offset)
                .take(visible_buffer_length)
                .enumerate()
                .map(|(i, network_item)| {
                    (
                        (data_window_length         
                            .saturating_sub(i+1)) as f64,
                            
                        byte_to_kb(network_item.tx()) as f64,
                    )
                })
                .collect();

        let rx_visible_data: Vec<(f64, f64)> = self.queue_state
                .iter()
                .rev()
                .skip(data_window_offset)
                .take(visible_buffer_length)
                .enumerate()
                .map(|(i, network_item)| {
                    (
                        (data_window_length     
                            .saturating_sub(i+1)) as f64,

                        byte_to_kb(network_item.rx()) as f64,
                    )
                })
                .collect();

        // setting lower and upper y bounds
        let y_lower_bound = 0.0;
        let y_upper_bound = tx_visible_data
            .iter()
            .chain(rx_visible_data.iter())
            .map(|tuple| tuple.1)
            .fold(0.0, f64::max);
        let y_upper_bound = y_upper_bound * y_upper_bound_scale_factor;
    
        // setting lower and upper x bounds
        let x_lower_bound = 0.0;
        let x_upper_bound = data_window_length.saturating_sub(1) as f64;

        // setting x-axis
        let x_axis = Axis::default()
            .bounds([x_lower_bound, x_upper_bound])
            .labels(vec![
                Span::raw(format!("{}idx", x_upper_bound)),
                Span::raw(format!("{}idx", x_lower_bound)),
                ])
            .labels_alignment(Alignment::Right);

        // setting y-axis
        let y_axis = Axis::default()
            .bounds([y_lower_bound, y_upper_bound])
            .labels(vec![
                Span::raw(format!("{}", y_lower_bound)),
                Span::raw(format!("{}", y_upper_bound)),
            ])
            .labels_alignment(Alignment::Right);

        // setting datasets
        let datasets = vec![
            Dataset::default()
                .data(&tx_visible_data)
                .graph_type(GraphType::Line)
                .marker(Marker::Braille)
                .style(Style::new().light_blue()),
            Dataset::default()
                .data(&rx_visible_data)
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

        // set chart
        let chart_title = " Network ";
        let chart = Chart::new(datasets)
            .block(Block::default()
                .borders(Borders::LEFT|Borders::BOTTOM|Borders::RIGHT)
                .style(block_style)
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

        // render chart
        f.render_widget(chart, vertical_chunks[1]);

        let network_item = if let Some(item) = self.queue_state.back() {
            item
        }
        else {
            &NetworkItem::default()
        };
        
        // legend
        let tx_per_s = network_item.tx() / ms_to_s(refresh_rate);
        let rx_per_s = network_item.rx() / ms_to_s(refresh_rate);
        let tx_legend = format!("TX/s {}KB :: TOTAL TX {}MB", byte_to_kb(tx_per_s), network_item.total_tx().byte_to_mb());
        let rx_legend = format!("RX/s {}KB :: TOTAL RX {}MB", byte_to_kb(rx_per_s), network_item.total_rx().byte_to_mb());

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