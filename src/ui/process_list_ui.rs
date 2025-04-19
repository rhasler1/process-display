pub mod process_list_ui {
    use ratatui::{
        Frame,
        prelude::*,
        widgets::{block::*, *},
    };
    use process_list::ListIterator;
    use crate::config::ThemeConfig;

    pub fn draw_process_list<'a>(
        f: &mut Frame,
        area: Rect,
        visible_items: ListIterator<'a>,
        follow_selection: bool,
        focus: bool,
        theme_config: ThemeConfig,
    ) {
        let follow_flag = follow_selection;
        let header_style = theme_config.list_header;
        let select_style = theme_config.item_select;
        let select_follow_style = theme_config.item_select_follow;
        let default_style = theme_config.item_style;
        let out_of_focus_style = theme_config.component_out_of_focus;
        let in_focus_style = theme_config.component_in_focus;

        // setting header
        let header = ["", "Pid", "Name", "CPU (%)", "Memory (B)", "Runtime (s)", "Status"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(
                if focus {
                    in_focus_style
                }
                else {
                    out_of_focus_style
                }
            )
            .height(1);

        // setting rows
        let rows = visible_items
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
                    Cell::from(item.run_time().to_string()),
                    Cell::from(item.status()),
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
            Constraint::Length(20), // cpu usage
            Constraint::Length(20), // memory usage
            Constraint::Length(20), // run time
            Constraint::Length(20), // status
        ];

        // setting block information
        let block_title: &str = " Process List ";
        let block_style =
            if focus {
                in_focus_style
            }
            else {
                out_of_focus_style
            };

        // setting the table
        let table = Table::new(rows, widths)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(block_title))
            .style(block_style);

        // render
        f.render_widget(table, area);
    }

}