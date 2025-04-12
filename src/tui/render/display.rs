use super::app::Tab;
use crate::tui::render::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, List, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
};
use std::{path::PathBuf, str::FromStr};

impl App {
    /// Shortens the code necessary to set the color of a terminal element.
    pub fn get_color(&self, color: &str) -> Color {
        Color::from_str(color).unwrap_or(Color::Reset)
    }

    /// Turns '/home/USER' into '~' when displaying a path.
    pub fn format_display_path(&self, path: &PathBuf) -> String {
        let current_dir = path.to_string_lossy().to_string();
        if let Some(home) = dirs::home_dir() {
            let home_str = home.to_string_lossy();
            if current_dir.starts_with(&*home_str) {
                return format!("~{}", &current_dir[home_str.len()..]);
            }
        }
        current_dir
    }

    /// Draws the elements on the terminal.
    pub fn draw(&self, frame: &mut Frame) {
        //┌──────┐
        //│ VARS │
        //└──────┘

        let display_path = self.format_display_path(&self.file_browser.current_dir);
        let border = &self.config.colors.border;
        let highlight_color = &self.config.colors.highlight_color;
        let status = &self.config.colors.status;
        let _testing_color = "#DDE1FF";

        //┌────────┐
        //│ LAYOUT │
        //└────────┘

        let [top, info, middle, bottom] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let [top_left, top_center, top_right] = Layout::horizontal([
            Constraint::Length(15),
            Constraint::Min(0),
            Constraint::Length(15),
        ])
        .areas(top);

        //┌────────────┐
        //│ BLOCK VARS │
        //└────────────┘

        let middle_block = Block::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.get_color(border)))
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(1));

        //┌───────────┐
        //│ RENDERING │
        //└───────────┘

        // TOP LEFT
        frame.render_widget(self.top_left(), top_left);
        // TOP CENTER
        frame.render_widget(self.top_center(), top_center);
        // TOP RIGHT
        frame.render_widget(self.top_right(), top_right);
        // PROGRESS BAR
        frame.render_widget(self.progress_bar(), bottom);
        // STATUS
        match self.tab {
            Tab::Playlist => match self.audio.get_len() {
                0 => {
                    frame.render_widget(
                        Paragraph::new(Line::from(vec![Span::styled(
                            "playlist is empty",
                            Style::default().fg(self.get_color(status)),
                        )]))
                        .block(Block::new())
                        .alignment(Alignment::Center),
                        info,
                    );
                }
                1 => {
                    frame.render_widget(
                        Paragraph::new(Line::from(vec![Span::styled(
                            "playlist (1 item)",
                            Style::default().fg(self.get_color(status)),
                        )]))
                        .block(Block::new())
                        .alignment(Alignment::Center),
                        info,
                    );
                }
                _ => {
                    frame.render_widget(
                        Paragraph::new(Line::from(vec![Span::styled(
                            format!("playlist ({} items)", self.audio.get_len()),
                            Style::default().fg(self.get_color(status)),
                        )]))
                        .block(Block::new())
                        .alignment(Alignment::Center),
                        info,
                    );
                }
            },
            Tab::Browser => {
                frame.render_widget(
                    Paragraph::new(Line::from(vec![Span::styled(
                        format!("{}", display_path),
                        Style::default().fg(self.get_color(status)),
                    )]))
                    .block(Block::new())
                    .alignment(Alignment::Center),
                    info,
                );
            }
        }
        // MIDDLE
        match self.tab {
            Tab::Playlist => {
                frame.render_widget(
                    Paragraph::new("queue info here")
                        .centered()
                        .block(middle_block),
                    middle,
                );
            }
            Tab::Browser => {
                frame.render_stateful_widget(
                    List::new(self.file_browser.list_items())
                        .block(middle_block)
                        .highlight_style(Style::default().fg(self.get_color(highlight_color))),
                    middle,
                    &mut self.file_browser.list_state.clone(),
                );
                frame.render_stateful_widget(
                    Scrollbar::new(ScrollbarOrientation::VerticalRight),
                    frame.area().inner(Margin {
                        vertical: 1,
                        horizontal: 0,
                    }),
                    &mut ScrollbarState::new(self.file_browser.entries.len())
                        .position(self.file_browser.selected),
                );
            }
        }
    }
}
