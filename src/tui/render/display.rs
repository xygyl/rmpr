use super::app::Tab;
use crate::tui::render::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    symbols::{border, line::THICK},
    text::{Line, Span},
    widgets::{Block, Borders, LineGauge, List, Padding, Paragraph},
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
        let album = &self.config.colors.album;
        let artist = &self.config.colors.artist;
        let border = &self.config.colors.border;
        let highlight_color = &self.config.colors.highlight_color;
        let options = &self.config.colors.options;
        let paused = &self.config.colors.paused;
        let seekbar_filled = &self.config.colors.seekbar_filled;
        let seekbar_unfilled = &self.config.colors.seekbar_unfilled;
        let status = &self.config.colors.status;
        let tab_selected = &self.config.colors.tab_selected;
        let tab_unselected = &self.config.colors.tab_unselected;
        let timestamp = &self.config.colors.timestamp;
        let title = &self.config.colors.title;
        let track_num = &self.config.colors.track_num;
        let volume = &self.config.colors.volume;
        let year = &self.config.colors.year;
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
            .border_set(border::THICK)
            .border_style(Style::default().fg(self.get_color(border)))
            .padding(Padding::horizontal(1));

        //┌───────────┐
        //│ RENDERING │
        //└───────────┘

        // TOP LEFT
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(vec![Span::styled(
                    match self.audio.is_empty() {
                        true => String::new(),
                        false => {
                            format!(
                                "{:.0}:{:02.0}/{}",
                                self.audio.sink_pos() / 60, // Minutes
                                self.audio.sink_pos() % 60, // Seconds
                                // Seperate function since the display could be None
                                self.data.display_duration_display() // Total time
                            )
                        }
                    },
                    Style::default().fg(self.get_color(timestamp)),
                )]),
                Line::from(vec![Span::styled(
                    format!(
                        "{}",
                        match self.audio.is_empty() {
                            true => {
                                "stopped"
                            }
                            false => match self.audio.paused {
                                true => "paused",
                                false => "playing",
                            },
                        }
                    ),
                    Style::default().fg(self.get_color(paused)),
                )]),
            ])
            .block(
                Block::new()
                    .borders(Borders::TOP | Borders::BOTTOM | Borders::LEFT)
                    .border_set(border::THICK)
                    .border_style(Style::default().fg(self.get_color(border)))
                    .padding(Padding::horizontal(1)),
            )
            .alignment(Alignment::Left),
            top_left,
        );
        // TOP CENTER
        frame.render_widget(
            Paragraph::new(match self.audio.is_empty() {
                true => {
                    vec![Line::from("")]
                }
                false => {
                    vec![
                        Line::from(vec![
                            Span::styled(
                                format!("{}", self.data.display_artist()),
                                Style::default().fg(self.get_color(artist)),
                            ),
                            Span::from(" "),
                            Span::styled(
                                format!("{}", self.data.display_title()),
                                Style::default().fg(self.get_color(title)),
                            ),
                        ]),
                        Line::from(vec![
                            Span::styled(
                                format!("{}", self.data.display_album()),
                                Style::default().fg(self.get_color(album)),
                            ),
                            Span::from(" "),
                            Span::styled(
                                format!("{}", self.data.display_year()),
                                Style::default().fg(self.get_color(year)),
                            ),
                            Span::from(" "),
                            Span::styled(
                                format!("{}", self.data.display_track_number()),
                                Style::default().fg(self.get_color(track_num)),
                            ),
                        ]),
                    ]
                }
            })
            .block(
                Block::new()
                    .borders(Borders::TOP | Borders::BOTTOM)
                    .border_set(border::THICK)
                    .border_style(Style::default().fg(self.get_color(border)))
                    .title_bottom(
                        Line::from(vec![
                            Span::styled("┫", self.get_color(border)),
                            Span::styled(
                                " 1 ",
                                match self.tab {
                                    Tab::Browser => {
                                        Style::default().fg(self.get_color(tab_selected))
                                    }
                                    _ => Style::default().fg(self.get_color(tab_unselected)),
                                },
                            ),
                            Span::styled(
                                " 2 ",
                                match self.tab {
                                    Tab::Playlist => {
                                        Style::default().fg(self.get_color(tab_selected))
                                    }
                                    _ => Style::default().fg(self.get_color(tab_unselected)),
                                },
                            ),
                            Span::styled("┣", self.get_color(border)),
                        ])
                        .centered(),
                    ),
            )
            .alignment(Alignment::Center),
            top_center,
        );

        // TOP RIGHT
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(vec![Span::styled(
                    format!("{}%", self.audio.vol),
                    Style::default().fg(self.get_color(volume)),
                )]),
                Line::from(vec![
                    Span::styled(format!("-"), Style::default().fg(self.get_color(options))),
                    Span::styled(format!("-"), Style::default().fg(self.get_color(options))),
                    Span::styled(format!("-"), Style::default().fg(self.get_color(options))),
                    Span::styled(format!("-"), Style::default().fg(self.get_color(options))),
                    Span::styled(format!("-"), Style::default().fg(self.get_color(options))),
                    Span::styled(format!("-"), Style::default().fg(self.get_color(options))),
                ]),
            ])
            .block(
                Block::new()
                    .borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT)
                    .border_set(border::THICK)
                    .border_style(Style::default().fg(self.get_color(border)))
                    .padding(Padding::horizontal(1)),
            )
            .alignment(Alignment::Right),
            top_right,
        );
        // STATUS
        match self.tab {
            Tab::Playlist => {
                frame.render_widget(
                    Paragraph::new(Line::from(vec![Span::styled(
                        format!("Playlist ({} items)", self.audio.get_len()),
                        Style::default().fg(self.get_color(status)),
                    )]))
                    .block(Block::new())
                    .alignment(Alignment::Center),
                    info,
                );
            }
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
            }
        }
        // PROGRESS BAR
        frame.render_widget(
            LineGauge::default()
                .block(Block::new())
                .label("")
                .line_set(THICK)
                .ratio(self.prog_bar)
                .filled_style(Style::default().fg(self.get_color(&seekbar_filled)))
                .unfilled_style(Style::default().fg(self.get_color(&seekbar_unfilled))),
            bottom,
        );
    }
}
