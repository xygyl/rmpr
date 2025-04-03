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
    pub fn draw(&self, frame: &mut Frame) {
        let border = &self.config.colors.border;
        let currently_playing = &self.config.colors.currently_playing;
        let directory_path = &self.config.colors.directory_path;
        let highlight_color = &self.config.colors.highlight_color;
        let muted = &self.config.colors.muted;
        let paused = &self.config.colors.paused;
        let playback_speed = &self.config.colors.playback_speed;
        let volume = &self.config.colors.volume;

        let testing_color = "#DDE1FF";

        // Displays HOME as ~ instead of /home/user
        let current_dir = self.file_browser.current_dir.to_string_lossy().to_string();
        let display_path = if let Some(home) = dirs::home_dir() {
            let home_str = home.to_string_lossy();
            if current_dir.starts_with(&*home_str) {
                format!("~{}", &current_dir[home_str.len()..]) // Takes a slice of the string that starts at the end of home_str and ends at the length of the path
            } else {
                current_dir
            }
        } else {
            current_dir
        };

        ////////////////////////////
        // LEFT BLOCK BORDER VECS //
        ////////////////////////////
        // Displays the CWD
        let top_left = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!("{}", display_path),
                Style::default().fg(Color::from_str(directory_path).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(border).unwrap())),
        ]);

        // Displays the current play speed
        let top_right = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!("x{:<4}", (self.audio.play_speed as f32) / 100.0),
                Style::default().fg(Color::from_str(playback_speed).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(border).unwrap())),
        ]);

        // Displays the title of the currently playing song
        let bottom_left = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!("{}", self.data.display_title()),
                Style::default().fg(Color::from_str(currently_playing).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(border).unwrap())),
        ]);

        // For metadata and stats display testing
        let bottom_center = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_track_number()),
                Style::default().fg(Color::from_str(testing_color).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_artist()),
                Style::default().fg(Color::from_str(testing_color).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_album()),
                Style::default().fg(Color::from_str(testing_color).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_year()),
                Style::default().fg(Color::from_str(testing_color).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_duration_display()),
                Style::default().fg(Color::from_str(testing_color).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(border).unwrap())),
        ]);

        // Displays audio playing information
        let bottom_right = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!("{}", if self.audio.paused { "P" } else { "-" }),
                Style::default().fg(Color::from_str(paused).unwrap()),
            ),
            Span::styled(
                format!("{}", if self.audio.muted { "M" } else { "-" }),
                Style::default().fg(Color::from_str(muted).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!("{:>3}%", self.audio.vol),
                Style::default().fg(Color::from_str(volume).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(border).unwrap())),
        ]);

        /////////////////////////
        // RENDERING VARIABLES //
        /////////////////////////
        let left_block = Block::new()
            .border_set(border::THICK)
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::from_str(border).unwrap()))
            .title_top(top_left.left_aligned())
            .title_top(top_right.right_aligned())
            .title_bottom(bottom_left.left_aligned())
            .title_bottom(bottom_center.centered())
            .title_bottom(bottom_right.right_aligned());

        let list = List::new(self.file_browser.list_items())
            .block(left_block)
            .highlight_style(Style::default().fg(Color::from_str(highlight_color).unwrap()));

        let middle_right = symbols::border::Set {
            top_left: symbols::line::THICK_HORIZONTAL_DOWN,
            bottom_left: symbols::line::THICK_HORIZONTAL_UP,
            ..symbols::border::THICK
        };

        let right_block = Block::new()
            .border_set(middle_right)
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM | Borders::LEFT)
            .border_style(Style::default().fg(Color::from_str(border).unwrap()));

        let block1 = Block::bordered()
            .border_style(Style::default().fg(Color::from_str(border).unwrap()))
            .border_set(border::THICK);

        let progress_top = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(border).unwrap())),
            Span::styled(
                format!(
                    "{}/{:.0}",
                    self.audio.sink_pos(),
                    self.data.duration_as_secs.unwrap_or(0.0)
                ),
                Style::default().fg(Color::from_str(testing_color).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(border).unwrap())),
        ]);

        let progress_block = Block::bordered()
            .border_style(Style::default().fg(Color::from_str(border).unwrap()))
            .border_set(border::THICK)
            .title_bottom(progress_top.centered());

        let progress = Gauge::default()
            .gauge_style(Style::default().fg(Color::from_str(directory_path).unwrap()))
            .ratio(
                self.audio.sink_pos() as f64
                    / self.data.duration_as_secs.unwrap_or(f64::MIN_POSITIVE),
            )
            .use_unicode(true)
            .label("")
            .block(progress_block.clone());

        //////////////////////////
        // LAYOUT AND RENDERING //
        //////////////////////////
        let vertical = Layout::vertical([
            Constraint::Length(7),
            Constraint::Min(0),
            Constraint::Length(3),
        ]);
        let [top, mid, bot] = vertical.areas(frame.area());

        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [left, right] = horizontal.areas(mid);

        frame.render_stateful_widget(list, left, &mut self.file_browser.list_state.clone());
        frame.render_widget(progress, bot);
        frame.render_widget(
            Paragraph::new("queue info here").block(right_block.clone()),
            right,
        );
        frame.render_widget(
            Paragraph::new("metadata info here").block(block1.clone()),
            top,
        );
    }
}
