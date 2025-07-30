use std::collections::BTreeMap;
use ratatui::Frame;
use ratatui::layout::Position;
use ratatui::prelude::*;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState};
use std::str::FromStr;
use anyhow::{Ok, Result};
use crate::components::utils::data_window::DataWindow;
use crate::input::*;
use super::EventState;
use crate::components::common_nav;
use crate::services::sysinfo_service::SysInfoService;
use crate::components::utils::selection::UISelection;
use crate::components::*;
use crate::models::bounded_queue_model::BoundedQueueModel;
use crate::models::items::cpu_item::CpuItem;
use crate::config::Config;

#[derive(PartialEq, Clone, Copy)]
pub enum ColorWheel {
    LightRed,
    Blue,
    Cyan,
    Green,
    LightGreen,
    LightYellow,
    LightMagenta,
}

impl Default for ColorWheel {
    fn default() -> Self {
        ColorWheel::LightRed
    }
}

impl ColorWheel {
    const ALL: [ColorWheel; 7] = [
        ColorWheel::LightRed,
        ColorWheel::Blue,
        ColorWheel::Cyan,
        ColorWheel::Green,
        ColorWheel::LightGreen,
        ColorWheel::LightYellow,
        ColorWheel::LightMagenta,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ColorWheel::LightRed => "lightred",
            ColorWheel::Blue => "blue",
            ColorWheel::Cyan => "cyan",
            ColorWheel::Green => "green",
            ColorWheel::LightGreen => "lightgreen",
            ColorWheel::LightYellow => "lightyellow",
            ColorWheel::LightMagenta => "lightmagenta",
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

pub enum Focus {
    Chart,
    CPUList,
}

pub struct CPUComponent {
    cpus: BTreeMap<usize, BoundedQueueModel<CpuItem>>,
    selection_state: UISelection,
    selection_offset: usize,
    //data_window_time_scale: u64,
    data_window: DataWindow,
    chart_area: Option<Rect>,
    list_area: Option<Rect>,
    focus: Focus,
    config: Config,
}

impl CPUComponent {
    pub fn new(config: Config, sysinfo: &SysInfoService) -> Self {
        let mut cpus: BTreeMap<usize, BoundedQueueModel<CpuItem>> = BTreeMap::new();

        let data_window_capacity = ( config.max_time_span() / config.refresh_rate() ) as usize;
        let data_window_length = ( config.default_time_span() / config.refresh_rate() ) as usize;
        let data_window_offset = 0;
        let data_window = DataWindow::new(data_window_offset, data_window_length, data_window_capacity).unwrap();
        
        let focus: Focus = Focus::CPUList;
        //let data_window_time_scale = config.default_time_span();
        //let capacity = ( config.max_time_span() / config.refresh_rate() ) as usize;

        for cpu in sysinfo.get_cpus() {
            let id = cpu.id();

            let perf_q = cpus.entry(id).or_insert_with(|| {
                BoundedQueueModel::new(data_window_capacity)
            });

            // passes ownership
            perf_q.add_item(cpu);
        }

        let selection_state = if cpus.len() > 0 { UISelection::new(Some(0)) } else { UISelection::new(None) };
        // this is the offset between the SelectionState Selection (ui selection) & cpu_selection (backend)
        // this offset is present because an option to display ALL cpus is present in the ui list that is
        // not present in the CPU list
        let selection_offset = 1;

        let chart_area = None;
        let list_area = None;

        Self {
            cpus,
            selection_state,
            selection_offset,
            data_window,
            chart_area,
            list_area,
            focus,
            config,
        }       
    }

    fn handle_mouse_click_on_chart(&mut self, click_x: u16, click_y: u16) -> bool {
        if self.chart_area.is_none() { return false; }
        let chart_area = self.chart_area.unwrap();

        if chart_area.contains(Position {x: click_x, y: click_y}) {
            self.focus = Focus::Chart;
            return true
        }

        false
    }

    fn handle_mouse_click_on_list(&mut self, click_x: u16, click_y: u16) -> bool {
        if self.list_area.is_none() { return false; }
        let list_area = self.list_area.unwrap();

        if list_area.contains(Position { x: click_x, y: click_y }) {
            self.focus = Focus::CPUList;
            return true
        }
        
        return false
    }

    // has ownership
    pub fn update(&mut self, sysinfo: &SysInfoService) {
        let capacity = ( self.config.max_time_span() / self.config.refresh_rate() ) as usize;

        for cpu in sysinfo.get_cpus() {
            let id = cpu.id();

            let perf_q = self.cpus.entry(id).or_insert_with(|| {
                BoundedQueueModel::new(capacity)
            });

            // passes ownership
            perf_q.add_item(cpu);
        }
    }

    fn handle_move_selection(&mut self, dir: MoveSelection) {
        let len = self.cpus.len();
        let offset = self.selection_offset;
        self.selection_state.move_selection(dir, len + offset);
    } 
}

impl Component for CPUComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState> {
        let key_config = &self.config.key_config;
        let time_step = self.config.time_step();
        let refresh_rate = self.config.refresh_rate();
        let idx_step  = time_step / refresh_rate;

        // key event to change selection / data window
        match self.focus {
            Focus::CPUList => {
                if let Some(dir) = common_nav(key, key_config) {
                    self.handle_move_selection(dir);
                    return Ok(EventState::Consumed)
                }
            }
            Focus::Chart => {
                if key == key_config.move_down {
                    self.data_window.zoom_out(idx_step as usize);
                    //self.data_window_time_scale = min(self.data_window_time_scale.saturating_add(self.config.time_step()), self.config.max_time_span());
                    return Ok(EventState::Consumed)
                }
                if key == key_config.move_up {
                    self.data_window.zoom_in(idx_step as usize);
                    //self.data_window_time_scale = max(self.data_window_time_scale.saturating_sub(self.config.time_step()), self.config.max_time_span());
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

            }
        }

        // key event to move focus
        if key == key_config.filter {
            match self.focus {
                Focus::CPUList => { self.focus = Focus::Chart }
                Focus::Chart => { self.focus = Focus::CPUList }
            }
            return Ok(EventState::Consumed)
        }

        Ok(super::EventState::NotConsumed)
    }

    fn mouse_event(&mut self, mouse: Mouse) -> Result<EventState> {
        let time_step = self.config.time_step();
        let refresh_rate = self.config.refresh_rate();
        let idx_step  = time_step / refresh_rate;

        match self.focus {
            Focus::CPUList => {
                match mouse.kind {
                    MouseKind::ScrollDown => { self.handle_move_selection(MoveSelection::Down); }
                    MouseKind::ScrollUp => { self.handle_move_selection(MoveSelection::Up); }
                    //TODO: LeftClick
                    _ => {}
                }    
            }
            Focus::Chart => {
                match mouse.kind {
                    MouseKind::ScrollDown => {
                        self.data_window.zoom_out(idx_step as usize);
                        //self.data_window_time_scale = min(self.data_window_time_scale.saturating_add(self.config.time_step()), self.config.max_time_span());
                        return Ok(EventState::Consumed)
                    }
                    MouseKind::ScrollUp => { 
                        self.data_window.zoom_in(idx_step as usize);
                        //self.data_window_time_scale = max(self.data_window_time_scale.saturating_sub(self.config.time_step()), self.config.max_time_span());
                        return Ok(EventState::Consumed)
                    }
                    _ => {}
                }
            }
        }

        // move focus
        if matches!(mouse.kind, MouseKind::LeftClick) {
            if self.handle_mouse_click_on_chart(mouse.column, mouse.row) {
                return Ok(EventState::Consumed)
            }
            if self.handle_mouse_click_on_list(mouse.column, mouse.row) {
                return Ok(EventState::Consumed)
            }
        }

        Ok(EventState::NotConsumed)
    }
}

impl DrawableComponent for CPUComponent {
    // draw function has some magic numbers relating to render position: TODO-research a fix/better approach
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()> {
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),                    // chart
                Constraint::Length(20),                 // list
            ]).split(area);
        
        // saving chart and list area for mouse clicks
        self.chart_area = Some(horizontal_chunks[0]);
        self.list_area = Some(horizontal_chunks[1]);

        let data_window_length = self.data_window.window_length();
        let data_window_offset = self.data_window.window_offset();

        // containers
        let mut all_data: Vec<(u32, Vec<(f64, f64)>)> = Vec::new(); // collect all data ensuring references live long enough to be drawn by `datasets`
        let mut datasets: Vec<Dataset> = Vec::new();                // holds references to data to be drawn

        // get max index of a queue (they are all the same)
        /*let perf_q_max_idx = self.cpus
            .get(&0)
            .map(|q| q.capacity().saturating_sub(1))
            .unwrap_or(0); */

        // The UICPUList will look like:
        // All              ui_selection=0      cpu_selection=None
        // Global Usage     ui_selection=1      cpu_selection=0
        // CPU 0            ui_selection=2      cpu_selection=1
        // CPU 1            ui_selection=3      cpu_selection=2
        // ...              ...
        // This means len(UICPUList) = 1 + len(cpus)

        // set cpu selection
        let cpu_selection = if let Some(selection) = self.selection_state.selection {
            if selection == 0 { None }
            else {Some(selection.saturating_sub(self.selection_offset))}
  
        }
        else { None };

        // iterate over cpus
        if let Some(selection) = self.selection_state.selection {
            if selection == 0 {
                // display all
                for (id, queue) in self.cpus
                    .iter()
                    {
                        let data: Vec<(f64, f64)> = queue
                            .iter()
                            .rev()
                            .skip(data_window_offset)
                            .take(data_window_length)
                            .enumerate()
                            .map(|(i, cpu_item)| (
                                (data_window_length
                                    .saturating_sub(i+1)) as f64,
                                cpu_item.usage() as f64)
                            )
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
                        .skip(data_window_offset)
                        .take(data_window_length)
                        .enumerate()
                        .map(|(i, cpu_item)| (
                            (data_window_length
                                .saturating_sub(i+1)) as f64,
                            cpu_item.usage() as f64))
                        .collect();

                    all_data.push((id as u32, data));
                }
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
        let mut names: Vec<(usize, String)> = self.cpus
            .iter()
            .map(|(key, queue)| {
                let usage = queue.back().unwrap().usage();

                let label = if *key == 0 {
                    String::from("Global")
                }
                else {
                    format!("CPU {}", key.saturating_sub(1))
                };

                (*key, format!("{:<7} {:.2}", label, usage))
                //ListItem::new(title).style(Color::from_str(ColorWheel::from_index(*key).as_str()).unwrap())
            })
            .collect();
        // insert All option into UI list
        let all_option = format!("{:<7} {}", String::from("All"), String::from("%"));
        // assigning random key to all for coloring, colorwheel::7783 => magenta
        names.insert(0, (7783, all_option));

        // map Vec<String> to Vec<ListItem>
        let names: Vec<ListItem> = names
            .iter()
            .enumerate()
            .map(|(i, (key, name))| {
                if Some(i) == self.selection_state.selection {
                    ListItem::new(format!("-> {}", name)).style(Color::from_str(ColorWheel::from_index(*key).as_str()).unwrap())
                }
                else {
                    ListItem::new(format!("   {}", name)).style(Color::from_str(ColorWheel::from_index(*key).as_str()).unwrap())
                }
            }).collect();

        // render chart
        let chart = Chart::new(datasets)
            .block(
                {
                    if focused && matches!(self.focus, Focus::Chart) {
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" CPU ")
                            .style(self.config.theme_config.style_border_focused)
                    }
                    else {
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" CPU ")
                            .style(self.config.theme_config.style_border_not_focused)
                    }
                }
            )
            .x_axis(
                Axis::default()
                    .bounds([0.0,  data_window_length.saturating_sub(1) as f64])
                    .labels(vec![Span::raw(format!("{}s", data_window_length.saturating_sub(1))), Span::raw("now")])
                    .labels_alignment(Alignment::Right),
            )
            .y_axis(
                Axis::default()
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::raw("0%"),
                        Span::raw("50"),
                        Span::raw("100"),
                    ])
                    .labels_alignment(Alignment::Right),
            );
            
        f.render_widget(chart, horizontal_chunks[0]);

        // render cpu list
        let mut list_state = ListState::default();
        list_state.select(self.selection_state.selection);
        
        let cpu_list = List::new(names)
            .scroll_padding(horizontal_chunks[1].height as usize / 2)
            .block(
                if focused && matches!(self.focus, Focus::CPUList) {
                    Block::default().borders(Borders::ALL).style(self.config.theme_config.style_border_focused)
                }
                else {
                    Block::default().borders(Borders::ALL).style(self.config.theme_config.style_border_not_focused)
                }
            );

        f.render_stateful_widget(cpu_list, horizontal_chunks[1], &mut list_state);

        Ok(())
    }
}