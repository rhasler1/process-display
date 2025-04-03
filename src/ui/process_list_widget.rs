//todo: determine what is needed to render process list widget
// 1. chunk of list
use process_list::ListIterator;
use crate::{components::process::Focus, config::ThemeConfig};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

pub struct ProcessListWidget<'a> {
    pub visible_items: ListIterator<'a>,
    pub focus: Focus,
    pub follow_selection: bool,
    pub theme_config: ThemeConfig,
}

impl<'a> ProcessListWidget<'a> {
    pub fn draw(self, f: &mut Frame, area: Rect, _focus: bool) {
        let follow_flag = self.follow_selection;
        let header_style = self.theme_config.list_header;
        //let header_style = Style::default().fg(Color::Black).bg(Color::Gray);
        let select_style = self.theme_config.item_select;
        //let select_style = Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD);
        let select_follow_style = self.theme_config.item_select_follow;
        //let select_follow_style = Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED);
        let default_style = self.theme_config.item_style;
        //let default_style = Style::default().fg(Color::White);
        let out_of_focus_style = self.theme_config.component_out_of_focus;
        //let out_of_focus_style = Style::default().fg(Color::DarkGray);

        // setting header
        let header = ["", "Pid", "Name", "CPU Usage (%)", "Memory Usage (Bytes)"]
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

        // setting rows
        let rows = self.visible_items
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

                let cells: Vec<Cell> = vec![
                    if style == select_style || style == select_follow_style {
                        Cell::from(String::from("->"))
                    }
                    else {
                        Cell::from(String::from(""))
                    },
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
            Constraint::Length(2),
            Constraint::Length(10),
            Constraint::Length(50),
            Constraint::Length(20),
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
        f.render_widget(table, area);
    }
}