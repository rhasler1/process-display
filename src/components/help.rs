use anyhow::Ok;
use anyhow::Result;
use itertools::Itertools;
use crate::input::*;
use ratatui::{
    Frame,
    prelude::*,
    widgets::*,
};
use crate::config::Config;
use super::command::CommandInfo;
use super::EventState;
use super::DrawableComponent;
use super::Component;

pub struct HelpComponent {
    cmds: Vec<CommandInfo>,
    visible: bool,
    selection: u16,
    pub config: Config,
}

impl HelpComponent {
    pub const fn new(config: Config) -> Self {
        Self {
            cmds: vec![],
            visible: false,
            selection: 0,
            config,
        }
    }

    pub fn set_commands(&mut self, cmds: Vec<CommandInfo>) {
        self.cmds = cmds
            .into_iter()
            .filter(|e| !e.text.hide_help)
            .collect::<Vec<_>>();
    }

    fn scroll_selection(&mut self, inc: bool) {
        let mut new_selection = self.selection;

        new_selection = if inc {
            new_selection.saturating_add(1)
        }
        else {
            new_selection.saturating_sub(1)
        };
        new_selection = new_selection.max(0);
        
        self.selection = new_selection.min(self.cmds.len().saturating_sub(1) as u16);

    }

    fn get_text(&self, width: usize) -> Vec<Line> {
        let mut txt: Vec<Line> =  Vec::new();

        let mut processed = 0;

        for (key, group) in &self.cmds.iter().group_by(|e| e.text.group) {
            txt.push(
                Line::from(
                    Line::styled(
                        key.to_string(),
                        Style::default().add_modifier(Modifier::REVERSED),
                    )
                )
            );

            for command_info in group {
                let is_selected = self.selection == processed;
                processed += 1;

                txt.push(
                    Line::from(
                        Line::styled(
                            format!(" {}{:width$}", command_info.text.name, width),
                            if is_selected {
                                Style::default().bg(Color::Blue)
                            }
                            else {
                                Style::default()
                            },
                        )
                    )
                );
            }
        }

        txt
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;

        Ok(())
    }

}

impl Component for HelpComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState> {
        if self.visible {
            if key == self.config.key_config.exit {
                self.hide();
                return Ok(EventState::Consumed);
            }
            else if key == self.config.key_config.move_down {
                self.scroll_selection(true);
                return Ok(EventState::Consumed);
            }
            else if key == self.config.key_config.move_up {
                self.scroll_selection(false);
                return Ok(EventState::Consumed);
            }
        }
        else if key == self.config.key_config.open_help {
            self.show()?;
            return Ok(EventState::Consumed);
        }
        Ok(EventState::NotConsumed)
    }

    fn mouse_event(&mut self, _mouse: Mouse) -> Result<EventState> {
        Ok(EventState::NotConsumed)
    }
}

impl DrawableComponent for HelpComponent {
    fn draw(&mut self, f: &mut Frame, _area: Rect, _focused: bool) -> Result<()> {
        if self.visible {
            const SIZE: (u16, u16) = (65, 24);
            let scroll_threshold = SIZE.1 / 3;
            let scroll = self.selection.saturating_sub(scroll_threshold);

            let area = Rect::new(
                (f.size().width.saturating_sub(SIZE.0)) / 2,
                (f.size().height.saturating_sub(SIZE.1)) / 2,
                SIZE.0.min(f.size().width),
                SIZE.1.min(f.size().height),
            );

            f.render_widget(Clear, area);

            f.render_widget(
                Block::default()
                    .title("Help")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick),
                area,
            );

            let chunks = Layout::default()
                .vertical_margin(1)
                .horizontal_margin(1)
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
                .split(area);

            f.render_widget(
                Paragraph::new(self.get_text(chunks[0].width as usize)).scroll((scroll, 0)),
                chunks[0],
            );

            f.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(
                    format!("process-display"),
                    Style::default(),
                )]))
                .alignment(Alignment::Right),
                chunks[1],
            );
        }
        Ok(())
    }
}
