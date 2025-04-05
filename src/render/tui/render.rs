use crate::render::tui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    symbols::{self, border},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, Padding, Paragraph},
    Frame,
};
use std::str::FromStr;

impl App {
    pub fn update(&mut self) {
        if self.audio.sink_len() == 0 {
            self.prog_bar = 0.0;
            return;
        }
        // Displays in milliseconds / milliseconds for higher resolution seekbar
        self.prog_bar = (self.audio.sink_pos_millis() as f64
            / (self.data.duration_as_secs.unwrap() * 1000.0))
            .clamp(0.0, 1.0);
    }

    fn get_color(color: &str) -> Color {
        Color::from_str(color).unwrap_or(Color::Reset)
    }

    pub fn draw(&self, frame: &mut Frame) {
        let album = &self.config.colors.album;
        let artist = &self.config.colors.artist;
        let border = &self.config.colors.border;
        let highlight_color = &self.config.colors.highlight_color;
        let options = &self.config.colors.options;
        let paused = &self.config.colors.paused;
        let seekbar = &self.config.colors.seekbar;
        let timestamp = &self.config.colors.timestamp;
        let title = &self.config.colors.title;
        let track_num = &self.config.colors.track_num;
        let volume = &self.config.colors.volume;
        let year = &self.config.colors.year;

        let _testing_color = "#DDE1FF";

        ///////////////
        // TEXT VECS //
        ///////////////

        let top_mid_block_vec = vec![
            Line::from(vec![
                Span::styled(
                    format!("{}", self.data.display_artist()),
                    Style::default().fg(App::get_color(artist)),
                ),
                Span::styled(" ┃ ", Style::default().fg(App::get_color(border))),
                Span::styled(
                    format!("{}", self.data.display_title()),
                    Style::default().fg(App::get_color(title)),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!(" {} ", self.data.display_album()),
                    Style::default().fg(App::get_color(album)),
                ),
                Span::styled("┃", Style::default().fg(App::get_color(border))),
                Span::styled(
                    format!(" {} ", self.data.display_year()),
                    Style::default().fg(App::get_color(year)),
                ),
                Span::styled("┃", Style::default().fg(App::get_color(border))),
                Span::styled(
                    format!(" {} ", self.data.display_track_number()),
                    Style::default().fg(App::get_color(track_num)),
                ),
            ]),
        ];

        let top_left_block_vec = vec![
            Line::from(vec![Span::styled(
                match self.audio.sink_len() {
                    0 => String::new(),
                    _ => {
                        format!(
                            "{:.0}:{:02.0}/{}",
                            self.audio.sink_pos() / 60, // Minutes
                            self.audio.sink_pos() % 60, // Seconds
                            // Seperate function since the display could be None
                            self.data.display_duration_display() // Total time
                        )
                    }
                },
                Style::default().fg(App::get_color(timestamp)),
            )]),
            Line::from(vec![Span::styled(
                format!(
                    "{}",
                    match self.audio.sink_len() {
                        0 => {
                            "stopped"
                        }
                        _ => match self.audio.paused {
                            true => "paused",
                            false => "playing",
                        },
                    }
                ),
                Style::default().fg(App::get_color(paused)),
            )]),
        ];

        let top_right_block_vec = vec![
            Line::from(vec![Span::styled(
                format!("{:>3}%", self.audio.vol),
                Style::default().fg(App::get_color(volume)),
            )]),
            Line::from(vec![
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
            ]),
        ];

        /////////////////////////
        // RENDERING VARIABLES //
        /////////////////////////

        let top_center_left_block = Block::bordered()
            .borders(Borders::TOP | Borders::BOTTOM | Borders::LEFT)
            .border_set(border::THICK)
            .border_style(Style::default().fg(App::get_color(border)))
            .padding(Padding::horizontal(1));

        let top_center_right_block = Block::bordered()
            .borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT)
            .border_set(border::THICK)
            .border_style(Style::default().fg(App::get_color(border)))
            .padding(Padding::horizontal(1));

        let top_center_mid_block = Block::bordered()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_set(border::THICK)
            .border_style(Style::default().fg(App::get_color(border)));

        let mid_left_block = Block::new()
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_set(border::THICK)
            .border_style(Style::default().fg(App::get_color(border)))
            .padding(Padding::horizontal(1));

        let list = List::new(self.file_browser.list_items())
            .block(mid_left_block)
            .highlight_style(Style::default().fg(App::get_color(highlight_color)));

        let mid_right_set = symbols::border::Set {
            top_left: symbols::line::THICK_HORIZONTAL_DOWN,
            bottom_left: symbols::line::THICK_HORIZONTAL_UP,
            ..symbols::border::THICK
        };

        let mid_right_block = Block::new()
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM | Borders::LEFT)
            .border_set(mid_right_set)
            .border_style(Style::default().fg(App::get_color(border)));

        let progress_block = Block::bordered()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_set(border::THICK)
            .border_style(Style::default().fg(App::get_color(border)));

        let progress = Gauge::default()
            .block(progress_block)
            .label("")
            .ratio(self.prog_bar)
            .use_unicode(true)
            .gauge_style(Style::default().fg(App::get_color(seekbar)));

        //////////////////////////
        // LAYOUT AND RENDERING //
        //////////////////////////

        let vertical = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [top, mid, bot] = vertical.areas(frame.area());

        let horizontal_top = Layout::horizontal([
            Constraint::Length(15),
            Constraint::Min(0),
            Constraint::Length(15),
        ]);
        let horizontal_mid =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [top_left, top_center, top_right] = horizontal_top.areas(top);
        let [mid_left, mid_right] = horizontal_mid.areas(mid);

        match self.audio.sink_len() {
            0 => frame.render_widget(Paragraph::new("").block(top_center_mid_block), top_center),
            _ => frame.render_widget(
                Paragraph::new(top_mid_block_vec)
                    .block(top_center_mid_block)
                    .alignment(Alignment::Center),
                top_center,
            ),
        }
        frame.render_widget(
            Paragraph::new(top_left_block_vec)
                .block(top_center_left_block)
                .alignment(Alignment::Left),
            top_left,
        );
        frame.render_widget(
            Paragraph::new(top_right_block_vec)
                .block(top_center_right_block)
                .alignment(Alignment::Right),
            top_right,
        );
        frame.render_stateful_widget(list, mid_left, &mut self.file_browser.list_state.clone());
        frame.render_widget(
            Paragraph::new("queue info here")
                .centered()
                .block(mid_right_block),
            mid_right,
        );
        frame.render_widget(progress, bot);
    }
}
