use std::collections::BTreeMap;
use ratatui::prelude::*;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState};
use std::str::FromStr;
use anyhow::Ok;
use bounded_queue::{BoundedQueue, CpuItem};
use crate::config::Config;
use super::{Component, DrawableComponent};

#[derive(PartialEq, Clone, Copy)]
pub enum ColorWheel {
    Red,
    Blue,
    Cyan,
    Green,
    LightGreen,
    Magenta,
}

impl Default for ColorWheel {
    fn default() -> Self {
        ColorWheel::Red
    }
}

impl ColorWheel {
    const ALL: [ColorWheel; 6] = [
        ColorWheel::Red,
        ColorWheel::Blue,
        ColorWheel::Cyan,
        ColorWheel::Green,
        ColorWheel::LightGreen,
        ColorWheel::Magenta,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ColorWheel::Red => "red",
            ColorWheel::Blue => "blue",
            ColorWheel::Cyan => "cyan",
            ColorWheel::Green => "green",
            ColorWheel::LightGreen => "lightgreen",
            ColorWheel::Magenta => "magenta",
        }
    }

    pub fn rotate(&mut self) {
        if let Some(idx) = Self::ALL.iter().position(|c| c == self) {
            let next_idx = (idx + 1) % Self::ALL.len();
            *self = Self::ALL[next_idx];
        }
    }

    pub fn from_index(index: usize) -> Self {
        Self::ALL[index % Self::ALL.len()]
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Default)]
pub struct CPUComponent {
    cpus: BTreeMap<usize, BoundedQueue<CpuItem>>,
    ui_selection: usize,
    config: Config,
}

impl CPUComponent {
    // has ownership
    pub fn update(&mut self, cpus: Vec<CpuItem>) {
        for cpu in cpus {
            let id = cpu.id();

            let perf_q = self.cpus.entry(id).or_insert_with(|| {
                BoundedQueue::new(self.config.events_per_min() as usize)
            });

            // passes ownership
            perf_q.add_item(cpu);
        }
    }
}

impl Component for CPUComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<super::EventState> {
        if key.code == self.config.key_config.move_down {
            if self.ui_selection < self.cpus.len() { // this works b/c we prepend ALL to the drawn list
                self.ui_selection = self.ui_selection.saturating_add(1);
            }
            return Ok(super::EventState::Consumed);
        }
        if key.code == self.config.key_config.move_up {
            self.ui_selection = self.ui_selection.saturating_sub(1);
            return Ok(super::EventState::Consumed);
        }
        
        Ok(super::EventState::NotConsumed)
    }
}

impl DrawableComponent for CPUComponent {
    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect, focused: bool) -> anyhow::Result<()> {
        // split screen
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),                    // chart
                Constraint::Length(16),                // list
            ]).split(area);

        // containers
        let mut all_data: Vec<(u32, Vec<(f64, f64)>)> = Vec::new(); // collect all data ensuring references live long enough to be drawn by `datasets`
        let mut datasets: Vec<Dataset> = Vec::new();                // holds references to data to be drawn

        // get max index of a queue (they are all the same)
        let perf_q_max_idx = self.cpus
            .get(&0)
            .map(|q| q.capacity().saturating_sub(1))
            .unwrap_or(0); 

        // The UICPUList will look like:
        // All              ui_selection=0      cpu_selection=None
        // Global Usage     ui_selection=1      cpu_selection=0
        // CPU 0            ui_selection=2      cpu_selection=1
        // CPU 1            ui_selection=3      cpu_selection=2
        // ...              ...
        // This means len(UICPUList) = 1 + len(cpus)

        // set cpu selection
        let cpu_selection = if self.ui_selection == 0 {
            None
        }
        else {
            Some(self.ui_selection.saturating_sub(1))
        };


        // iterate over cpus
        if self.ui_selection == 0 {
            // display all
            for (id, queue) in self.cpus
                .iter()
                {
                    let data: Vec<(f64, f64)> = queue
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, item)| ((perf_q_max_idx - i) as f64, item.usage() as f64))
                        .collect();
                    
                    all_data.push((*id as u32, data));
                }
        }
        else {
            let id = cpu_selection.unwrap();            // unwrap should be safe here

            if let Some(queue) = self.cpus.get(&id) {
                let data: Vec<(f64, f64)> = queue
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, item)| ((perf_q_max_idx - i) as f64, item.usage() as f64))
                    .collect();

                all_data.push((id as u32, data));
            }
        }

        // populate datasets for drawing chart
        for (id, data) in all_data.iter() {
            datasets.push(
                Dataset::default()
                    .data(data)
                    .graph_type(GraphType::Line)
                    .marker(symbols::Marker::Braille)
                    .style(Color::from_str(ColorWheel::from_index(*id as usize).as_str())?)
            );
        }

        // populate names for UIList to draw cpu list
        let mut names: Vec<ListItem> = self.cpus
            .iter()
            .map(|(key, queue)| {
                let usage = queue.back().unwrap().usage();

                let label = if *key == 0 {
                    String::from("Global")
                }
                else {
                    format!("CPU {}", key.saturating_sub(1))
                };

                let title = format!("{:<7} {:.2}", label, usage);

                ListItem::new(title).style(Color::from_str(ColorWheel::from_index(*key).as_str()).unwrap())
            })
            .collect();

        // insert All option into UI list
        let title = format!("{:<7} {}", String::from("All"), String::from("%"));
        names.insert(0, ListItem::new(title));

        // render chart
        let chart = Chart::new(datasets)
            .block(
                {
                    if !focused {
                        Block::default().borders(Borders::ALL).title(" CPU % ").style(self.config.theme_config.style_border_not_focused)
                    }
                    else {
                        Block::default().borders(Borders::ALL).title(" CPU % ").style(self.config.theme_config.style_border_focused)
                    }
                }
            )

            .x_axis(
                Axis::default()
                    .bounds([0.0, self.config.events_per_min().saturating_sub(1) as f64])
                    .labels(vec![Span::raw(format!("-{}s", self.config.min_as_s())), Span::raw("now")])
                    .labels_alignment(Alignment::Right),
            )
            .y_axis(
                Axis::default()
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::raw("0"),
                        Span::raw("50"),
                        Span::raw("100"),
                    ])
                    .labels_alignment(Alignment::Right),
            );
            
        f.render_widget(chart, horizontal_chunks[0]);

        // render cpu list
        let mut list_state = ListState::default();
        
        list_state.select(Some(self.ui_selection));
        
        let cpu_list = List::new(names)
            .scroll_padding(horizontal_chunks[1].height as usize / 2)
            .block(
                if !focused {
                    Block::default().borders(Borders::ALL).style(self.config.theme_config.style_border_not_focused)
                }
                else {
                    Block::default().borders(Borders::ALL).style(self.config.theme_config.style_border_focused)
                }
            )
            .highlight_style(
                if !focused {
                    self.config.theme_config.style_item_selected_not_focused
                }
                else {
                    self.config.theme_config.style_item_selected
                }
            );

        f.render_stateful_widget(cpu_list, horizontal_chunks[1], &mut list_state);

        Ok(())
    }
}