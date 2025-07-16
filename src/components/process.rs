use anyhow::{Ok, Result};
use crossterm::event::KeyEvent;
use ratatui::{Frame, prelude::*,};
use crate::services::{sysinfo_service::SysInfoService, ListProvider};
use crate::config::{Config, KeyConfig};
use crate::components::{common_nav, DrawableComponent, Component, EventState, Refreshable};
use crate::components::{utils::{selection::UISelection, vertical_scroll::VerticalScroll}, filter::FilterComponent};
use crate::states::vec_state::VecState;
use crate::models::items::process_item::{ProcessItem, ProcessItemSortOrder};

#[derive(PartialEq, Clone)]
pub enum Focus {
    Filter,
    List,
}

pub struct ProcessComponent {
    vec_state: VecState<ProcessItem, ProcessItemSortOrder>,
    ui_selection: UISelection,
    sort: Option<ProcessItemSortOrder>,
    scroll: VerticalScroll,
    filter_component: FilterComponent,
    focus: Focus,
    pub config: Config,
}

impl ProcessComponent {
    pub fn new(config: Config, sysinfo: &SysInfoService) -> Self {
        let processes: Vec<ProcessItem> = sysinfo.fetch_items();
        let ui_selection: UISelection = if processes.is_empty() { UISelection::new(None) } else { UISelection::new(Some(0)) };
        let state_selection: Option<usize> = ui_selection.selection;
        let filter: Option<String> = None;
        let sort: Option<ProcessItemSortOrder> = None;
        let vec_state: VecState<ProcessItem, ProcessItemSortOrder> = VecState::new(processes, state_selection, sort.clone(), filter);

        let scroll: VerticalScroll = VerticalScroll::new();
        let filter_component: FilterComponent = FilterComponent::new(config.clone());
        let focus: Focus = Focus::List;

        Self {
            vec_state,
            ui_selection,
            sort,
            scroll,
            filter_component,
            focus,
            config,
        }
    }
}

impl<S> Refreshable<S> for ProcessComponent
where
    S: ListProvider<ProcessItem>
{
    fn refresh(&mut self, service: &S) {
        let processes: Vec<ProcessItem> = service.fetch_items();
        self.vec_state.replace(processes);

        let len = self.vec_state.view_indices().len();
        if len == 0 {
            self.ui_selection.set_selection(None);
            self.vec_state.set_selection(self.ui_selection.selection);
            return;
        }

        // ui_selection iterates over vec_state.view_indices()
        if let Some(ui_selection) = self.ui_selection.selection {
            let max_idx = len.saturating_sub(1);

            if ui_selection > max_idx {
                self.ui_selection.set_selection(Some(max_idx));
                let idx = self.vec_state.view_indices().get(max_idx).cloned();
                self.vec_state.set_selection(idx);
            }
            else {
                let idx = self.vec_state.view_indices().get(ui_selection).cloned();
                self.vec_state.set_selection(idx);
            }
        }
    }
}


impl Component for ProcessComponent {
    fn event(&mut self, key: KeyEvent) -> Result<EventState> {
        if key.code == self.config.key_config.filter &&
            matches!(self.focus,Focus::List)
        {
            self.focus = Focus::Filter;
            return Ok(EventState::Consumed)
        }

        if matches!(self.focus, Focus::Filter) {

            if self.filter_component.event(key)?.is_consumed() {
                self.vec_state.set_filter(self.filter_component.filter_contents());

                if self.vec_state.view_indices().len() > 0 {
                    // set ui_selection to beginning of view_indices
                    self.ui_selection.set_selection(Some(0));
                    // set vec_state selection to the index at view_indices[0]
                    let idx = self.vec_state.view_indices().get(0).cloned();
                    self.vec_state.set_selection(idx);
                }

                return Ok(EventState::Consumed)
            }
            
            if key.code == self.config.key_config.enter {
                self.focus = Focus::List;
                return Ok(EventState::Consumed)
            }
        }

        if matches!(self.focus, Focus::List) {
            if let Some(move_dir) = common_nav(key, &self.config.key_config) {
                let len = self.vec_state.view_indices().len();
                self.ui_selection.move_selection(move_dir, len);                    // if len == 0, ui_selection.selection is set to None here

                if let Some(ui_selection) = self.ui_selection.selection {
                    let idx = self.vec_state.view_indices().get(ui_selection).cloned();
                    self.vec_state.set_selection(idx);
                }
                else {
                    self.vec_state.set_selection(None);
                }
                
                return Ok(EventState::Consumed)
            }

            if let Some(sort_order) = process_sort(key, &self.config.key_config) {
                self.sort = Some(sort_order);
                self.vec_state.set_sort(self.sort.clone());

                if let Some(ui_selection) = self.ui_selection.selection {
                    let idx = self.vec_state.view_indices().get(ui_selection).cloned();
                    self.vec_state.set_selection(idx);
                }
                else {
                    self.vec_state.set_selection(None);
                }

                return Ok(EventState::Consumed)
            }

            if key.code == self.config.key_config.follow_selection {            // TODO: implement follow selection?
                return Ok(EventState::Consumed)
            }
        }
        
        Ok(EventState::NotConsumed)
    }
}

fn process_sort(key: KeyEvent, key_config: &KeyConfig) -> Option<ProcessItemSortOrder> {
    if key.code == key_config.sort_pid_inc { return Some(ProcessItemSortOrder::PidInc) }
    if key.code == key_config.sort_pid_dec { return Some(ProcessItemSortOrder::PidDec) }
    if key.code == key_config.sort_cpu_usage_inc { return Some(ProcessItemSortOrder::CpuUsageInc) }
    if key.code == key_config.sort_cpu_usage_dec { return Some(ProcessItemSortOrder::CpuUsageDec) }
    if key.code == key_config.sort_memory_usage_inc { return Some(ProcessItemSortOrder::MemoryUsageInc) }
    if key.code == key_config.sort_memory_usage_dec { return Some(ProcessItemSortOrder::MemoryUsageDec) }
    if key.code == key_config.sort_name_inc { return Some(ProcessItemSortOrder::NameInc) }
    if key.code == key_config.sort_name_dec { return Some(ProcessItemSortOrder::NameDec) }

    return None
}

impl DrawableComponent for ProcessComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()> {
        let horizontal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),        //list
                Constraint::Length(3),      //filter
            ]).split(area);

        let visible_list_height = horizontal_chunks[0].height.saturating_sub(3) as usize;

        // update vertical scroll
        let indices = self.vec_state.view_indices();
        let len = indices.len();
        self.ui_selection.selection.map_or_else(
            { ||
                self.scroll.reset()
            }, |idx| {
                self.scroll.update(idx, len, visible_list_height,);
        },);

        let visible_items = self.vec_state
            .iter_with_selection()
            .skip(self.scroll.get_top())
            .take(visible_list_height);

        draw_process_list(
            f, 
            horizontal_chunks[0], 
            visible_items,
            if focused {
                matches!(self.focus, Focus::List)
            } 
            else {
                false
            },
            self.config.theme_config.clone(),
            self.sort.clone(),
        );

        self.scroll.draw(
            f, 
            horizontal_chunks[0],
            if focused {
                matches!(self.focus, Focus::List)
            } 
            else {
                false
            },
        )?;
                
        self.filter_component.draw(
            f, 
            horizontal_chunks[1],
            if focused {
                matches!(self.focus, Focus::Filter)
            } 
            else {
                false
            },
        )?;

        Ok(())
    }
}

use ratatui::widgets::{block::*, *};
use crate::config::ThemeConfig;

fn draw_process_list<'a, I>(
    f: &mut Frame,
    area: Rect,
    visible_items: I,
    focus: bool,
    theme_config: ThemeConfig,
    sort_order: Option<ProcessItemSortOrder>,
)
where
    I: Iterator<Item = (usize, &'a ProcessItem, bool)>,
{
    let follow_flag = false;

    // setting header
    let header = ["",
        if matches!(sort_order, Some(ProcessItemSortOrder::PidInc)) { "PID ▲" }
        else if matches!(sort_order, Some(ProcessItemSortOrder::PidDec)) { "PID ▼" }
        else { "PID" },

        if matches!(sort_order, Some(ProcessItemSortOrder::NameInc)) { "Name ▲" } 
        else if matches!(sort_order, Some(ProcessItemSortOrder::NameDec)) { "Name ▼" }
        else { "Name" },

        if matches!(sort_order, Some(ProcessItemSortOrder::CpuUsageInc)) { "CPU (%) ▲" }
        else if matches!(sort_order, Some(ProcessItemSortOrder::CpuUsageDec)) { "CPU (%) ▼" }
        else { "CPU (%)" },

        if matches!(sort_order, Some(ProcessItemSortOrder::MemoryUsageInc)) { "Memory (MB) ▲" }
        else if matches!(sort_order, Some(ProcessItemSortOrder::MemoryUsageDec)) { "Memory (MB) ▼" }
        else { "Memory (MB)" },

        "Run (hh:mm:ss)",
        "Status",
        "Path"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(
            if focus {
                theme_config.style_border_focused
            }
            else {
                theme_config.style_border_not_focused
            }
        )
        .height(1);

    // setting rows
    let rows = visible_items
        .map(|(idx, item, selected)| {
            let style =
                if focus && selected && follow_flag {
                    theme_config.style_item_selected_followed
                }
                else if focus && selected && !follow_flag {
                    theme_config.style_item_selected
                }
                else if focus {
                    theme_config.style_item_focused
                }
                else if !focus && selected & follow_flag {
                    theme_config.style_item_selected_followed_not_focused
                }
                else if !focus && selected & !follow_flag {
                    theme_config.style_item_selected_not_focused
                }
                else {
                    theme_config.style_item_not_focused
                };

            let cells: Vec<Cell> = vec![
                if style == theme_config.style_item_selected ||
                    style == theme_config.style_item_selected_followed || 
                    style == theme_config.style_item_selected_followed_not_focused || 
                    style == theme_config.style_item_selected_not_focused {
                        Cell::from(String::from("->"))
                }
                else {
                    Cell::from(String::from(""))
                },
                Cell::from(item.pid().to_string()),
                Cell::from(item.name().to_string()),
                Cell::from(format!("{:.2}", item.cpu_usage())),
                Cell::from(format!("{:.2}", item.memory_usage()/1000000)),
                Cell::from(item.run_time_hh_mm_ss()),
                Cell::from(item.status()),
                Cell::from(item.path()),
            ];
            Row::new(cells).style(style)
        })
        .collect::<Vec<_>>();

    // setting the width constraints.
    let widths =
    vec![
        Constraint::Length(2),
        Constraint::Length(10), // pid
        Constraint::Length(50), // name
        Constraint::Length(15), // cpu usage
        Constraint::Length(15), // memory usage
        Constraint::Length(20), // run time
        Constraint::Length(15), // status
        Constraint::Min(0),     // path
    ];

    // setting block information
    let block_title: &str = " Process List ";
    let block_style =
        if focus {
            theme_config.style_border_focused
        }
        else {
            theme_config.style_border_not_focused
        };

    // setting the table
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(block_title))
        .style(block_style);

    // render
    f.render_widget(table, area);
}

#[cfg(test)]
mod test {
    // TODO: write tests
}
