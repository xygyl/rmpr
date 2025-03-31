use crate::tui::tui::App;
use ratatui::{
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List},
    Frame,
};
use std::str::FromStr;

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        let border = self.config.colors.border.clone();
        let currently_playing = self.config.colors.currently_playing.clone();
        let directory_path = self.config.colors.directory_path.clone();
        let highlight_color = self.config.colors.highlight_color.clone();
        let muted = self.config.colors.muted.clone();
        let paused = self.config.colors.paused.clone();
        let playback_speed = self.config.colors.playback_speed.clone();
        let volume = self.config.colors.volume.clone();

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

        // Displays the CWD
        let top_left = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("{}", display_path),
                Style::default().fg(Color::from_str(&directory_path).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

        // Displays the current play speed
        let top_right = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("x{:<4}", (self.audio.play_speed as f32) / 100.0),
                Style::default().fg(Color::from_str(&playback_speed).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

        // Displays the title of the currently playing song
        let bottom_left = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("{}", self.data.display_title()),
                Style::default().fg(Color::from_str(&currently_playing).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

        // See the songs in the queue and their order (for testing)
        /* let queue = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("{:?}", self.name),
                Style::default().fg(Color::from_str(&directory_path).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]); */

        // For metadata and stats display testing
        let bottom_center = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_track_number()),
                Style::default().fg(Color::from_str(&testing_color).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_artist()),
                Style::default().fg(Color::from_str(&testing_color).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_album()),
                Style::default().fg(Color::from_str(&testing_color).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_year()),
                Style::default().fg(Color::from_str(&testing_color).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!(" {} ", self.data.display_duration_display()),
                Style::default().fg(Color::from_str(&testing_color).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

        // Displays audio playing information
        let bottom_right = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("{}", if self.audio.paused { "P" } else { "-" }),
                Style::default().fg(Color::from_str(&paused).unwrap()),
            ),
            Span::styled(
                format!("{}", if self.audio.muted { "M" } else { "-" }),
                Style::default().fg(Color::from_str(&muted).unwrap()),
            ),
            Span::styled("┃", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("{:>3}%", self.audio.vol),
                Style::default().fg(Color::from_str(&volume).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

        let block = Block::bordered()
            .border_style(Style::default().fg(Color::from_str(&border).unwrap()))
            .border_set(border::THICK)
            .title_top(top_left.left_aligned())
            .title_top(top_right.right_aligned())
            .title_bottom(bottom_left.left_aligned())
            .title_bottom(bottom_center.centered())
            // .title_bottom(queue.centered())
            .title_bottom(bottom_right.right_aligned());

        let list = List::new(self.file_browser.list_items())
            .block(block)
            .highlight_style(Style::default().fg(Color::from_str(&highlight_color).unwrap()));

        frame.render_stateful_widget(
            list,
            frame.area(),
            &mut self.file_browser.list_state.clone(),
        )
    }
}
