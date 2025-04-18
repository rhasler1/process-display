use std::collections::BTreeMap;
use std::{collections::HashMap, hash::Hash};

use crossterm::style::style;
use ratatui::prelude::*;
use ratatui::style::palette::material::YELLOW;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState};

use anyhow::Ok;
use performance_queue::{PerformanceQueue, CpuItem};
use crate::config::Config;

use super::{Component, DrawableComponent};

#[derive(Default)]
pub struct CPUComponent {
    cpus: BTreeMap<usize, PerformanceQueue<CpuItem>>,
    selection: usize,
    config: Config,
}

impl CPUComponent {
    pub fn update(&mut self, cpus: &Vec<CpuItem>) {
        for cpu in cpus {
            let id = cpu.id();

            let perf_q = self.cpus.entry(id).or_insert_with(|| {
                PerformanceQueue::new(self.config.events_per_min() as usize)
            });

            perf_q.add_item(cpu);
        }
    }
}

impl Component for CPUComponent {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<super::EventState> {
        if key.code == self.config.key_config.move_down {
            if self.selection < self.cpus.len() { // this works b/c we prepend ALL to the drawn list
                self.selection = self.selection.saturating_add(1);
            }
            return Ok(super::EventState::Consumed);
        }
        if key.code == self.config.key_config.move_up {
            self.selection = self.selection.saturating_sub(1);
            return Ok(super::EventState::Consumed);
        }
        
        Ok(super::EventState::NotConsumed)
    }
}

/*
if selection = 0 display all
if selection = 1 display global
if selection = 2 display cpu 0
...

*/

impl DrawableComponent for CPUComponent {
    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect, _focused: bool) -> anyhow::Result<()> {
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(90),
                Constraint::Fill(1),
            ]).split(area);

        let mut all_data: Vec<Vec<(f64, f64)>> = Vec::new();
        let mut datasets: Vec<Dataset> = Vec::new(); // holds references to data to be drawn

        if self.selection == 0 {
            // display all
            for (_id, queue) in self.cpus.iter() {
                let data: Vec<(f64, f64)> = queue
                    .iter()
                    .enumerate()
                    .map(|(i, item)| (i as f64, item.usage() as f64))
                    .collect();
    
                all_data.push(data);
            }
        }
        else {
            // display selection-1 : accounting for All
            if let Some(queue) = self.cpus.get(&self.selection.saturating_sub(1)) {
                let data: Vec<(f64, f64)> = queue
                    .iter()
                    .enumerate()
                    .map(|(i, item)| (i as f64, item.usage() as f64))
                    .collect();

                all_data.push(data);
            }
        }

        for data in all_data.iter() {
            datasets.push(
                Dataset::default()
                    .name(format!("CPU"))
                    .data(data)
                    .graph_type(GraphType::Line)
                    .marker(symbols::Marker::Braille),
            );
        }

        let chart = Chart::new(datasets)
            .block(Block::default().borders(Borders::ALL).title("CPU Usage"))
            .x_axis(
                Axis::default()
                    .title("Time")
                    .bounds([0.0, self.config.events_per_min() as f64])
                    .labels(vec![Span::raw("0"), Span::raw("now")]),
            )
            .y_axis(
                Axis::default()
                    .title("%")
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::raw("0"),
                        Span::raw("50"),
                        Span::raw("100"),
                    ]),
            );

        f.render_widget(chart, horizontal_chunks[0]);
        
        // cpu list
        let mut names: Vec<ListItem> = self.cpus
            .iter()
            .map(|(key, _queue)| {
                let title = if *key == 0 {
                    format!("Global")
                }
                else {
                    format!("CPU {}", key.saturating_sub(1).to_string())
                };
                ListItem::new(title)
            })
            .collect();

        names.insert(0, ListItem::new(String::from("All")));

        let mut list_state = ListState::default();
        list_state.select(Some(self.selection));


        let cpu_list = List::new(names)
            .scroll_padding(horizontal_chunks[1].height as usize / 2)
            .block(Block::default().title("CPU List").borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        );
        
        f.render_stateful_widget(cpu_list, horizontal_chunks[1], &mut list_state);


        Ok(())
    }
}