use std::io;
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};
use super::{filter::FilterComponent, Component, EventState, DrawableComponent};
use super::utils::vertical_scroll::VerticalScroll;
use crate::process_structs::{common_nav, common_sort, process_list_items::ProcessListItems};
use crate::process_structs::process_list_item::ProcessListItem;
use crate::process_structs::process_list::ProcessList;
use crate::config::KeyConfig;

// The ProcessComponent can be navigated to focus on
// either a ProcessList <filtered/unfiltered> or
// FilterComponent.
#[derive(PartialEq)]
pub enum Focus {
    Filter,
    List,
    // Add Terminate variant; This way we can match focus when drawing the ProcessComponent and draw TerminateComponent.
}

pub struct ProcessComponent {
    focus: Focus,
    list: ProcessList,
    filter: FilterComponent,
    filtered_list: Option<ProcessList>,
    scroll: VerticalScroll,
    key_config: KeyConfig,
    // TODO: terminate: TerminateComponent
}

impl ProcessComponent {
    // New constructor.
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            focus: Focus::List,
            list: ProcessList::default(),
            filter: FilterComponent::default(),
            filtered_list: None,
            scroll: VerticalScroll::new(false, false),
            key_config: key_config,
        }
    }

    // This function is used to update the process lists. Presumeably there will
    // will always be an unfiltered list, it is updated without any conditions.
    // The filtered list is only updated if there is some filtered list.
    pub fn update(&mut self, new_processes: &Vec<ProcessListItem>) -> io::Result<()> {
        self.list.update(new_processes)?;
        if let Some(filtered_list) = self.filtered_list.as_mut() {
            // We first filter the new processes by the filter,
            let processes = ProcessListItems::new(new_processes);
            let filter_text = self.filter.input_str();
            let filtered_processes = processes.filter(filter_text);
            // then we update the filtered list with the new filtered processes.
            filtered_list.update(&filtered_processes.list_items)?;
        }
        Ok(())
    }

    // This function can be used to communicate the selected item's pid to the application.
    // Note: The function will always return None if the focus is on the filter.
    pub fn selected_pid(&self) -> Option<u32> {
        if matches!(self.focus, Focus::List) {
            if let Some(list) = self.filtered_list.as_ref() {
                return list.get_selected_pid()
            }
            else {
                return self.list.get_selected_pid()
            }
        }
        None
    }
}

impl Component for ProcessComponent {
    // Handle key events for ProcessComponent.
    fn event(&mut self, key: KeyEvent) -> io::Result<EventState> {
        //  If they key event is filter and the ProcessComponent Focus is on the List, then move the focus to Filter and return.
        if key.code == self.key_config.filter && self.focus == Focus::List {
            self.focus = Focus::Filter;
            return Ok(EventState::Consumed)
        }

        // If the ProcessComponent Focus is on the Filter, then attempt to set the filtered_list.
        // If the filter's input string is None, then set the filtered_list to None (no List to display),
        // else create the filtered_list calling list.filter(input_str).
        if matches!(self.focus, Focus::Filter) {
            self.filtered_list = if self.filter.input_str().is_empty() {
                None
            }
            else {
                Some(self.list.filter(self.filter.input_str()))
            };
        }

        // If the key event is enter and the focus is on the Filter, then change the focus to List and return.
        if key.code == self.key_config.enter && matches!(self.focus, Focus::Filter) {
            self.focus = Focus::List;
            return Ok(EventState::Consumed)
        }

        // The following if block contains key event tasks for when the filter is in focus.
        if matches!(self.focus, Focus::Filter) {
            // Check if the key input is to modify the filter.
            if self.filter.event(key)?.is_consumed() {
                return Ok(EventState::Consumed)
            }
        }

        //  The following if block contains key event tasks for when the process list is in focus.
        if matches!(self.focus, Focus::List) {
            // Check if the key input is to navigate the list. If there is some filtered list, then that is the
            // list we want to interact with, else interact with the unfiltered list.
            if list_nav(
                if let Some(list) = self.filtered_list.as_mut() {
                    list
                }
                else {
                    &mut self.list
                },
                key,
                &self.key_config
            ) {
                return Ok(EventState::Consumed);
            }

            // Check if the key is to change the follow_selection value.
            else if key.code == self.key_config.follow_selection {
                if let Some(filtered_list) = self.filtered_list.as_mut() {
                    filtered_list.change_follow_selection()?;
                }
                else {
                    self.list.change_follow_selection()?;
                }

                return Ok(EventState::Consumed)
            }

            // Check if the key input is to sort the list. If there is some filtered list, then that is the
            // list we want to interact with, else interact with unfiltered list.
            else if sort_list(
                if let Some(list) = self.filtered_list.as_mut() {
                    list
                }
                else {
                    &mut self.list
                },
                key,
                &self.key_config
            )? {
                return Ok(EventState::Consumed);
            }
        }

        Ok(EventState::NotConsumed)
    }
}

fn list_nav(list: &mut ProcessList, key: KeyEvent, key_config: &KeyConfig) -> bool {
    if let Some(move_dir) = common_nav(key, key_config) {
        list.move_selection(move_dir)
    }
    else {
        false
    }
}

fn sort_list(list: &mut ProcessList, key: KeyEvent, key_config: &KeyConfig) -> io::Result<bool> {
    if let Some(sort) = common_sort(key, key_config) {
        list.sort(sort)?; // 
        Ok(true)
    }
    else {
        Ok(false)
    }
}

impl DrawableComponent for ProcessComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> io::Result<()> {
        // Splitting the parameter area into two vertical chunks.
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // vertical chunk for TabComponent and FilterComponent : vertical_chunks[0]
                Constraint::Min(1), // vertical chunk for the ProcessList : vertical_chunks[1]
            ].as_ref())
            .split(area);

        // Splitting the vertical chunk for the TabComponent and FilterComponent into two horizontal chunks.
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // horizontal chunk for TabComponent : horizontal_chunks[0]
                Constraint::Percentage(50), // horizontal chunk for FilterComponent : horizontal_chunks[1]
            ].as_ref())
            .split(vertical_chunks[0]);

        // Drawing the filter component.
        self.filter.draw(f, horizontal_chunks[1], matches!(self.focus, Focus::Filter))?;

        // Setting the list height to the height of the vertical chunk for the process list; We are subtracting
        // two from the height to account for the area that will be taken up by the border around the list.
        let visual_list_height = (vertical_chunks[1].height.saturating_sub(2)) as usize;

        // Getting the list to display; If there is some filtered list display it, else display the unfiltered list.
        let list = if let Some(list) = self.filtered_list.as_ref() {
            list
        }
        else {
            &self.list
        };

        // Updating the scroll struct which calculates the position at the top of the displayed list.
        list.selection().map_or_else(
            { ||
                self.scroll.reset()
            }, |selection| {
                self.scroll.update(
                    selection,list.list_len(), visual_list_height
                );
            },
        );

        // Getting the boolean list.follow(); The follow_flag is used to differentiate between a selected item being followed(underlined) and not.
        let follow_flag = list.follow();

        // Different styles used to visually differentiate between components and focus.
        let header_style = Style::default().fg(Color::Black).bg(Color::Gray);
        let select_style = Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD);
        let select_follow_style = Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED);
        let default_style = Style::default().fg(Color::White);
        let out_of_focus_style = Style::default().fg(Color::DarkGray);

        // Setting the header.
        let header = ["Pid", "Name", "Cpu Usage (%)", "Memory Usage (Bytes)"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(
                if matches!(self.focus, Focus::List) {
                    header_style
                }
                else {
                    out_of_focus_style
                }
            )
            .height(1);

        // Setting the rows to display; The iterate function iterates starting from the value self.scroll.get_top() and a list_height number of times.
        // We don't iterate over the entire list everytime we draw the list, instead we only iterate over the portion that is being displayed.
        // See process_structs::list_items_iter::next for the implementation.
        let rows = list
            .iterate(self.scroll.get_top(), visual_list_height)
            .map(|(item, selected)| {
                let style =
                    if matches!(self.focus, Focus::List) && selected && follow_flag {
                        select_follow_style
                    }
                    else if matches!(self.focus, Focus::List) && selected && !follow_flag {
                        select_style
                    }
                    else if matches!(self.focus, Focus::List) {
                        default_style
                    }
                    else {
                        out_of_focus_style
                    };

                let cells = vec![
                    Cell::from(item.pid().to_string()),
                    Cell::from(item.name().to_string()),
                    Cell::from(item.cpu_usage().to_string()),
                    Cell::from(item.memory_usage().to_string()),
                ];

                Row::new(cells).style(style)
            })
            .collect::<Vec<_>>();

        // Setting the width constraints.
        let widths =
            vec![
                Constraint::Length(10),
                Constraint::Length(50),
                Constraint::Length(25),
                Constraint::Length(20),
            ];

        // Setting block information.
        let block_title: &str = "Process List";
        let block_style =
            if matches!(self.focus, Focus::List) {
                Style::default().fg(Color::White)
            }
            else {
                Style::default().fg(Color::DarkGray)
            };

        // Setting the table.
        let table = Table::new(rows, widths)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(block_title))
            .style(block_style);

        // Render.
        f.render_widget(table, vertical_chunks[1]);

        // Draw scrollbar.
        self.scroll.draw(f, vertical_chunks[1], false)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    // TODO
}