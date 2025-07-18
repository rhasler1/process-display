use anyhow::{Ok, Result};
use ratatui::{Frame, prelude::*};
use ratatui::widgets::{block::*, *};
use crate::services::VecProvider;
use crate::config::*;
use crate::components::{common_nav, Component, DrawableComponent, EventState, MoveSelection, Refreshable};
use crate::components::{utils::{selection::UISelection, vertical_scroll::VerticalScroll}, filter::FilterComponent};
use crate::states::vec_state::VecState;
use crate::models::items::process_item::{ProcessItem, ProcessItemSortOrder};

use crate::input::{Key, Mouse, MouseKind};

#[derive(PartialEq, Clone, Debug)]
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
    pub fn new<S>(config: Config, service: &S) -> Self
    where S: VecProvider<ProcessItem>
    {
        let processes: Vec<ProcessItem> = service.fetch_items();
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

    fn handle_move_selection(&mut self, dir: MoveSelection) {
        let len = self.vec_state.view_indices().len();
        // move ui selection by dir
        self.ui_selection.move_selection(dir, len);
        // map ui selection -> vec state index
        let vec_idx = self.compute_vec_state_idx();
        // update vec state selection to index
        self.vec_state.set_selection(vec_idx);
    }

    fn handle_refresh_selection(&mut self) {
        let len = self.vec_state.view_indices().len();
        let max_idx = len.saturating_sub(1);

        let new_ui_selection: Option<usize> = 
        if len == 0 {
            None
        }
        else {
            match self.ui_selection.selection {
                Some(ui_selection) => {
                    // if out of bounds, clamp to max index
                    if ui_selection > max_idx {
                        Some(max_idx)
                    }
                    else {
                        Some(ui_selection)
                    }
                }
                // occurs when going from empty -> non-empty list
                None => Some(0),
            }
        };

        self.ui_selection.set_selection(new_ui_selection);
        let vec_idx = self.compute_vec_state_idx();
        self.vec_state.set_selection(vec_idx);
    }

    fn handle_filter_selection(&mut self) {
        let len = self.vec_state.view_indices().len();

        let new_ui_selection: Option<usize> =
        if len == 0 {
            None
        }
        else {
            Some(0)
        };

        self.ui_selection.set_selection(new_ui_selection);
        let vec_idx = self.compute_vec_state_idx();
        self.vec_state.set_selection(vec_idx);
    }

    fn compute_vec_state_idx(&self) -> Option<usize> {
        // map ui_selection.selection to vec_state
        let vec_idx = self.ui_selection.selection
            .and_then(|ui_selection| self.vec_state.view_indices().get(ui_selection).cloned());

        vec_idx
    }
}

impl<S> Refreshable<S> for ProcessComponent
where
    S: VecProvider<ProcessItem>
{
    fn refresh(&mut self, service: &S) {
        let processes: Vec<ProcessItem> = service.fetch_items();
        self.vec_state.replace(processes);
        self.handle_refresh_selection();
    }
}


impl Component for ProcessComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState> {
        if key == self.config.key_config.filter &&
            matches!(self.focus,Focus::List)
        {
            self.focus = Focus::Filter;
            return Ok(EventState::Consumed)
        }

        if matches!(self.focus, Focus::Filter) {
            if self.filter_component.key_event(key)?.is_consumed() {
                self.vec_state.set_filter(self.filter_component.filter_contents());
                self.handle_filter_selection();
                return Ok(EventState::Consumed)
            }

            if key == self.config.key_config.enter {
                self.focus = Focus::List;
                return Ok(EventState::Consumed)
            }
        }

        if matches!(self.focus, Focus::List) {
            if let Some(move_dir) = common_nav(key, &self.config.key_config) {
                self.handle_move_selection(move_dir);
                return Ok(EventState::Consumed)
            }

            if let Some(sort_order) = match_sort_order_key(key, &self.config.key_config) {
                self.sort = Some(sort_order);
                self.vec_state.set_sort(self.sort.clone());
                self.handle_refresh_selection();                                        // handle_refresh() also works for handling sort
                return Ok(EventState::Consumed)
            }
        }
        
        Ok(EventState::NotConsumed)
    }

    fn mouse_event(&mut self, mouse: Mouse) -> Result<EventState> {
        match mouse.kind {
            MouseKind::ScrollDown | MouseKind::ScrollUp => {
                let dir = match mouse.kind {
                    MouseKind::ScrollDown => MoveSelection::Down,
                    MouseKind::ScrollUp => MoveSelection::Up,
                    _ => unreachable!(),                                                // safeguarded by conditional
                };

                self.handle_move_selection(dir);
                return Ok(EventState::Consumed);
            }
            _ => {}
        }

        Ok(EventState::NotConsumed)
    }
}

// maps key to ProcessItemSortOrder
fn match_sort_order_key(key: Key, key_config: &KeyConfig) -> Option<ProcessItemSortOrder> {
    if key == key_config.sort_pid_inc               { return Some(ProcessItemSortOrder::PidInc) }
    if key == key_config.sort_pid_dec               { return Some(ProcessItemSortOrder::PidDec) }
    if key == key_config.sort_cpu_usage_inc         { return Some(ProcessItemSortOrder::CpuUsageInc) }
    if key == key_config.sort_cpu_usage_dec         { return Some(ProcessItemSortOrder::CpuUsageDec) }
    if key == key_config.sort_memory_usage_inc      { return Some(ProcessItemSortOrder::MemoryUsageInc) }
    if key == key_config.sort_memory_usage_dec      { return Some(ProcessItemSortOrder::MemoryUsageDec) }
    if key == key_config.sort_name_inc              { return Some(ProcessItemSortOrder::NameInc) }
    if key == key_config.sort_name_dec              { return Some(ProcessItemSortOrder::NameDec) }

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

        // sub 3 here for table header (1 line) and borders (2 lines)
        let visible_list_height = horizontal_chunks[0].height.saturating_sub(4) as usize;

        // update vertical scroll
        let indices = self.vec_state.view_indices();
        let len = indices.len();
        self.ui_selection.selection.map_or_else(
            { ||
                self.scroll.reset()
            }, |idx| {
                self.scroll.update(idx, len, visible_list_height);
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

    // setting header
    let header_labels = [
        "",
        &header_with_sort(&sort_order, &ProcessItemSortOrder::PidInc, &ProcessItemSortOrder::PidDec, "PID"),
        &header_with_sort(&sort_order, &ProcessItemSortOrder::NameInc, &ProcessItemSortOrder::NameDec, "Name"),
        &header_with_sort(&sort_order, &ProcessItemSortOrder::CpuUsageInc, &ProcessItemSortOrder::CpuUsageDec, "CPU (%)"),
        &header_with_sort(&sort_order, &ProcessItemSortOrder::MemoryUsageInc, &ProcessItemSortOrder::MemoryUsageDec, "Memory (MB)"),
        "Run (hh:mm:ss)",
        "Status",
        "Path",
    ];

    let header = header_labels
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(if focus {theme_config.style_border_focused} else {theme_config.style_border_not_focused})
        .height(2);

    // setting rows
    let rows = visible_items
        .map(|(_idx, item, selected)| {
            let style = compute_row_style(focus, selected, &theme_config);

            let is_selected_style = [
                theme_config.style_item_selected,
                theme_config.style_item_selected_followed,
                theme_config.style_item_selected_followed_not_focused,
                theme_config.style_item_selected_not_focused,
            ].contains(&style);

            let cells = vec![
                Cell::from(if is_selected_style {"->"} else { "" }),
                Cell::from(item.pid().to_string()),
                Cell::from(item.name()),
                Cell::from(format!("{:.2}", item.cpu_usage())),
                Cell::from(format!("{:.2}", item.memory_usage()/1000000)),
                Cell::from(item.run_time_hh_mm_ss()),
                Cell::from(item.status()),
                Cell::from(item.path()),
            ];
            Row::new(cells).style(style)
        })
        .collect::<Vec<_>>();

    // setting width constraints
    let widths =
    vec![
        Constraint::Length(2),  // arrow
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
    let block_style = if focus { theme_config.style_border_focused } else { theme_config.style_border_not_focused };

    // setting the table
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(block_title))
        .style(block_style);

    // render
    f.render_widget(table, area);
}

// helper function for building header labels
fn header_with_sort(
    current: &Option<ProcessItemSortOrder>,
    inc: &ProcessItemSortOrder,
    dec: &ProcessItemSortOrder,
    base: &str,
) -> String {
    match current {
        Some(s) if s == inc => format!("{base} ▲"),
        Some(s) if s == dec => format!("{base} ▼"),
        _ => base.to_string(),
    }
}

// helper function for determining row style
fn compute_row_style(focus: bool, selected: bool, theme: &ThemeConfig) -> Style {
    match (focus, selected) {
        (true, true) => theme.style_item_selected,
        (true, _,) => theme.style_item_focused,
        (false, true) => theme.style_item_selected_not_focused,
        _ => theme.style_item_not_focused,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::services::VecProvider;
    use crate::models::items::process_item::ProcessItem;

    struct DummyService {
        // index into test data
        idx: usize
    }
    impl DummyService {
        fn new() -> Self {
            Self {
                idx: 0,
            }
        }

        // move idx to load new test data
        fn set(&mut self, idx: usize) {
            self.idx = idx;
        }
    }

    impl VecProvider<ProcessItem> for DummyService {
        fn fetch_items(&self) -> Vec<ProcessItem> {
            test_data(self.idx)
        }
    }
    

    #[test]
    fn test_constructor_with_data() {
        let mut service = DummyService::new();
        // 0=data, see test_data()
        service.set(0);
        let config = Config::default();
        let component = ProcessComponent::new(config.clone(), &service);

        // check vec_state contains expected number of items
        assert_eq!(component.vec_state.len(), test_data(service.idx).len());
        // check that ui_selection is Some(0) since there is data
        assert_eq!(component.ui_selection.selection, Some(0));
        // check that sort and filter are None
        assert!(component.sort.is_none());
        assert!(component.vec_state.filter().is_none());
        // check focus
        assert_eq!(component.focus, Focus::List);
    }

    #[test]
    fn test_constructor_with_no_data() {
        let mut service = DummyService::new();
        // 2=no data, see test_data()
        service.set(2);
        let config = Config::default();
        let component = ProcessComponent::new(config.clone(), &service);

        // check vec_state contains expected number of items
        assert_eq!(component.vec_state.len(), test_data(service.idx).len());
        // check that ui_selection is Some(0) since there is data
        assert_eq!(component.ui_selection.selection, None);
        // check that sort and filter are None
        assert!(component.sort.is_none());
        assert!(component.vec_state.filter().is_none());
        // check focus
        assert_eq!(component.focus, Focus::List);
    }

    #[test]
    fn test_handle_move_selection() {
        let mut service = DummyService::new();
        service.set(0);
        let config = Config::default();
        let mut component = ProcessComponent::new(config.clone(), &service);

        // testing boundaries
        component.handle_move_selection(MoveSelection::Up);
        assert_eq!(component.ui_selection.selection, Some(0));
        component.handle_move_selection(MoveSelection::Down);
        assert_eq!(component.ui_selection.selection, Some(1));
        component.handle_move_selection(MoveSelection::Bottom);
        assert_eq!(component.ui_selection.selection, Some(component.vec_state.view_indices().len().saturating_sub(1)));
        component.handle_move_selection(MoveSelection::Down);
        assert_eq!(component.ui_selection.selection, Some(component.vec_state.view_indices().len().saturating_sub(1)));
    }

    #[test]
    fn test_handle_refresh_selection() {
        let mut service = DummyService::new();
        service.set(0);
        let config = Config::default();
        let mut component = ProcessComponent::new(config.clone(), &service);

        // emulate refresh from non-empty to non-empty list
        let ui_selection = component.ui_selection.selection;
        service.set(1);
        component.refresh(&service);
        // ensure ui selection is same index (do not want ui selection to change unless out of bounds)
        assert_eq!(component.ui_selection.selection, ui_selection);
        assert!(component.vec_state.selection().is_some());

        // emulate refresh from non-empty to empty list
        service.set(2);
        component.refresh(&service);
        assert!(component.ui_selection.selection.is_none());
        assert!(component.vec_state.selection().is_none());

        // emulate refresh from empty to empty list
        service.set(2);
        component.refresh(&service);
        assert!(component.ui_selection.selection.is_none());
        assert!(component.vec_state.selection().is_none());

        // emulate refresh from empty to non-empty list
        service.set(0);
        component.refresh(&service);
        assert_eq!(component.ui_selection.selection, Some(0));
        assert!(component.vec_state.selection().is_some());
    }

    fn test_data(idx: usize) -> Vec<ProcessItem> {
        match idx {
            0 => {
                return vec![
                    ProcessItem::new(0, String::from("Discord"), 12.0, 12, 12, 12, 12, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(1, String::from("Slack"), 8.5, 15, 15, 15, 15, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(2, String::from("Chrome"), 25.3, 40, 40, 40, 40, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(3, String::from("iTerm"), 9.0, 9, 9, 9, 9, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(4, String::from("Spotify"), 7.2, 22, 22, 22, 22, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(5, String::from("VSCode"), 18.1, 35, 35, 35, 35, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(6, String::from("SystemUIServer"), 1.5, 5, 5, 5, 5, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(7, String::from("Dock"), 0.8, 3, 3, 3, 3, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(8, String::from("Finder"), 4.4, 18, 18, 18, 18, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(9, String::from("Discord-Helper"), 20.0, 20, 20, 20, 20, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(10, String::from("Photos"), 3.1, 12, 12, 12, 12, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(11, String::from("process-display"), 2.0, 2, 2, 2, 2, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(12, String::from("Mail"), 1.2, 7, 7, 7, 7, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(13, String::from("Calendar"), 0.6, 6, 6, 6, 6, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(14, String::from("Notes"), 0.4, 4, 4, 4, 4, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(15, String::from("Preview"), 0.9, 8, 8, 8, 8, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(16, String::from("Safari"), 11.0, 30, 30, 30, 30, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(17, String::from("Terminal"), 5.7, 10, 10, 10, 10, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(18, String::from("Activity Monitor"), 2.9, 14, 14, 14, 14, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(19, String::from("Xcode"), 14.3, 50, 50, 50, 50, String::from("Runnable"), String::from("test/")),
                ];
            }
            1 => {
                return vec![
                    ProcessItem::new(0, String::from("Discord"), 12.0, 12, 12, 12, 12, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(1, String::from("Slack"), 8.5, 15, 15, 15, 15, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(2, String::from("Chrome"), 25.3, 40, 40, 40, 40, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(3, String::from("iTerm"), 9.0, 9, 9, 9, 9, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(4, String::from("Spotify"), 7.2, 22, 22, 22, 22, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(5, String::from("VSCode"), 18.1, 35, 35, 35, 35, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(6, String::from("SystemUIServer"), 1.5, 5, 5, 5, 5, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(7, String::from("Dock"), 0.8, 3, 3, 3, 3, String::from("Sleeping"), String::from("test/")),
                    ProcessItem::new(8, String::from("Finder"), 4.4, 18, 18, 18, 18, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(9, String::from("Discord-Helper"), 20.0, 20, 20, 20, 20, String::from("Runnable"), String::from("test/")),
                    ProcessItem::new(10, String::from("Photos"), 3.1, 12, 12, 12, 12, String::from("Sleeping"), String::from("test/")),
                ];
            }
            _ => { return vec![]; }
        }
    }
}