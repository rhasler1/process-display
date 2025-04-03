use process_list::ListIterator;
use crate::config::ThemeConfig;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::*, *},
};

pub struct ProcessListUI<'a> {
    pub visible_items: ListIterator<'a>,
    pub follow_selection: bool,
    pub theme_config: ThemeConfig,
}

impl<'a> ProcessListUI<'a> {
    pub fn draw(self, f: &mut Frame, area: Rect, focus: bool) {
        let follow_flag = self.follow_selection;
        let header_style = self.theme_config.list_header;
        let select_style = self.theme_config.item_select;
        let select_follow_style = self.theme_config.item_select_follow;
        let default_style = self.theme_config.item_style;
        let out_of_focus_style = self.theme_config.component_out_of_focus;

        // setting header
        let header = ["", "Pid", "Name", "CPU Usage (%)", "Memory Usage (Bytes)"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(
                if focus {
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
                    if focus && selected && follow_flag {
                        select_follow_style
                    }
                    else if focus && selected && !follow_flag {
                        select_style
                    }
                    else if focus {
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
            if focus {
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