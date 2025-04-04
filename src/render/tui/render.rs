use crate::render::tui::app::App;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    symbols::{self, border},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, Paragraph},
    Frame,
};
use std::str::FromStr;

impl App {
    pub fn update(&mut self) {
        if self.audio.sink_len() == 0 {
            self.audio.clear_sink();
            self.seekbar = 0.0;
            return;
        }
        let duration = self.data.duration_as_secs.unwrap_or(1.0);
        self.seekbar = (self.audio.sink_pos() as f64 / duration).clamp(0.0, 1.0);
    }

    fn get_color(color: &str) -> Color {
        Color::from_str(color).unwrap_or(Color::Reset)
    }

    pub fn draw(&self, frame: &mut Frame) {
        let border = &self.config.colors.border;
        let artist = &self.config.colors.artist;
        let title = &self.config.colors.title;
        let album = &self.config.colors.album;
        let year = &self.config.colors.year;
        let track_num = &self.config.colors.track_num;
        let seekbar = &self.config.colors.seekbar;
        let highlight_color = &self.config.colors.highlight_color;
        let options = &self.config.colors.options;
        let paused = &self.config.colors.paused;
        let volume = &self.config.colors.volume;
        let timestamp = &self.config.colors.timestamp;

        let _testing_color = "#DDE1FF";

        ///////////////
        // TEXT VECS //
        ///////////////

        let top_mid_block_vec = vec![
            Line::from(vec![
                Span::styled("┫ ", Style::default().fg(App::get_color(border))),
                Span::styled(
                    format!("{}", self.data.display_artist()),
                    Style::default().fg(App::get_color(artist)),
                ),
                Span::styled(" ┃ ", Style::default().fg(App::get_color(border))),
                Span::styled(
                    format!("{}", self.data.display_title()),
                    Style::default().fg(App::get_color(title)),
                ),
                Span::styled(" ┣", Style::default().fg(App::get_color(border))),
            ]),
            Line::from(vec![
                Span::styled("┫", Style::default().fg(App::get_color(border))),
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
                Span::styled("┣", Style::default().fg(App::get_color(border))),
            ]),
        ];

        let top_right_block_vec = vec![
            Line::from(vec![Span::styled(
                format!("{:>3}% ", self.audio.vol),
                Style::default().fg(App::get_color(volume)),
            )]),
            Line::from(vec![
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("-"), Style::default().fg(App::get_color(options))),
                Span::styled(format!("- "), Style::default().fg(App::get_color(options))),
            ]),
        ];

        let top_left_block_vec = vec![
            Line::from(vec![Span::styled(
                match self.audio.sink_len() {
                    0 => String::new(),
                    _ => {
                        let pos = self.audio.sink_pos();
                        format!(" {}/{}", pos, self.data.display_duration_display())
                    }
                },
                Style::default().fg(App::get_color(timestamp)),
            )]),
            Line::from(vec![Span::styled(
                format!(
                    " {}",
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

        /////////////////////////
        // RENDERING VARIABLES //
        /////////////////////////

        let mid_left_block = Block::new()
            .border_set(border::THICK)
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_style(Style::default().fg(App::get_color(border)));

        let list = List::new(self.file_browser.list_items())
            .block(mid_left_block)
            .highlight_style(Style::default().fg(App::get_color(highlight_color)));

        let mid_right_set = symbols::border::Set {
            top_left: symbols::line::THICK_HORIZONTAL_DOWN,
            bottom_left: symbols::line::THICK_HORIZONTAL_UP,
            ..symbols::border::THICK
        };

        let mid_right_block = Block::new()
            .border_set(mid_right_set)
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM | Borders::LEFT)
            .border_style(Style::default().fg(App::get_color(border)));

        let top_center_left_block = Block::bordered()
            .border_set(border::THICK)
            .borders(Borders::TOP | Borders::BOTTOM | Borders::LEFT)
            .border_style(Style::default().fg(App::get_color(border)));

        let top_center_mid_block = Block::bordered()
            .border_set(border::THICK)
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(App::get_color(border)));

        let top_center_right_block = Block::bordered()
            .border_set(border::THICK)
            .borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT)
            .border_style(Style::default().fg(App::get_color(border)));

        let progress_block = Block::bordered()
            .border_set(border::THICK)
            .border_style(Style::default().fg(App::get_color(border)));

        let progress = Gauge::default()
            .gauge_style(Style::default().fg(App::get_color(seekbar)))
            .ratio(self.seekbar)
            .use_unicode(true)
            .label("")
            .block(progress_block);

        //////////////////////////
        // LAYOUT AND RENDERING //
        //////////////////////////

        let vertical = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(3),
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
                    .alignment(ratatui::layout::Alignment::Center),
                top_center,
            ),
        }
        frame.render_widget(
            Paragraph::new(top_left_block_vec)
                .block(top_center_left_block)
                .alignment(ratatui::layout::Alignment::Left),
            top_left,
        );
        frame.render_widget(
            Paragraph::new(top_right_block_vec)
                .block(top_center_right_block)
                .alignment(ratatui::layout::Alignment::Right),
            top_right,
        );
        frame.render_stateful_widget(list, mid_left, &mut self.file_browser.list_state.clone());
        frame.render_widget(
            Paragraph::new("queue info here").block(mid_right_block),
            mid_right,
        );
        frame.render_widget(progress, bot);
    }
}
