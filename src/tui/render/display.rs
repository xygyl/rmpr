use crate::tui::render::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    symbols::{self, border, line::THICK},
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

        let [mid_left, mid_right] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(middle);

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
        // TOP MIDDLE
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
                            Span::styled(" ┃ ", Style::default().fg(self.get_color(border))),
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
                            Span::styled(" ┃ ", Style::default().fg(self.get_color(border))),
                            Span::styled(
                                format!("{}", self.data.display_year()),
                                Style::default().fg(self.get_color(year)),
                            ),
                            Span::styled(" ┃ ", Style::default().fg(self.get_color(border))),
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
                    .border_style(Style::default().fg(self.get_color(border))),
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
        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(
                format!("{}", display_path),
                Style::default().fg(self.get_color(status)),
            )]))
            .block(Block::new())
            .alignment(Alignment::Center),
            info,
        );
        // MIDDLE LEFT
        frame.render_stateful_widget(
            List::new(self.file_browser.list_items())
                .block(
                    Block::new()
                        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                        .border_set(border::THICK)
                        .border_style(Style::default().fg(self.get_color(border)))
                        .padding(Padding::horizontal(1)),
                )
                .highlight_style(Style::default().fg(self.get_color(highlight_color))),
            mid_left,
            &mut self.file_browser.list_state.clone(),
        );
        // MIDDLE RIGHT
        frame.render_widget(
            Paragraph::new("queue info here").centered().block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::Set {
                        top_left: symbols::line::THICK_HORIZONTAL_DOWN,
                        bottom_left: symbols::line::THICK_HORIZONTAL_UP,
                        ..symbols::border::THICK
                    })
                    .border_style(Style::default().fg(self.get_color(border))),
            ),
            mid_right,
        );
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
