use crate::browser::FileBrowser;
use crate::config::ConfigData;
use crate::file_data::FileData;
use crate::metadata_manager::MetadataQueue;
use crate::{config::load_config, input_handler::InputHandler};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List},
    DefaultTerminal, Frame,
};
use std::{env, io, path::PathBuf, str::FromStr};

/// Runs the TUI application
pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let current_dir = env::current_dir()?;
    let mut app = App::new(current_dir)?;
    let res = app.run(&mut terminal);
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(res?)
}

/// The main application
pub struct App {
    config: ConfigData,
    meta_manager: MetadataQueue,
    file_browser: FileBrowser,
    audio: InputHandler,
    data: FileData,
    exit: bool,
}

impl App {
    pub fn new(initial_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let music_dir = load_config().directories.music_directory;

        let final_dir = if music_dir.exists() {
            music_dir
        } else {
            initial_dir
        };

        Ok(Self {
            config: load_config(),
            meta_manager: MetadataQueue::new(),
            file_browser: FileBrowser::new(final_dir),
            audio: InputHandler::new()?,
            data: FileData::new(),
            exit: false,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.file_browser.update_entries()?;
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
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

        let top_left = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("{}", display_path),
                Style::default().fg(Color::from_str(&directory_path).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

        let top_right = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("x{:<4}", (self.audio.play_speed as f32) / 100.0),
                Style::default().fg(Color::from_str(&playback_speed).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

        let bottom_left = Line::from(vec![
            Span::styled("┫", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!("{}", self.data.display_title()),
                Style::default().fg(Color::from_str(&currently_playing).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

        // For metadata display testing
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
            Span::styled("┃", Style::default().fg(Color::from_str(&border).unwrap())),
            Span::styled(
                format!(" {} ", self.audio.sink_pos()),
                Style::default().fg(Color::from_str(&testing_color).unwrap()),
            ),
            Span::styled("┣", Style::default().fg(Color::from_str(&border).unwrap())),
        ]);

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

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let speed_delta = self.config.controls.speed_delta;
        let audio_delta = self.config.controls.audio_delta;

        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

            KeyCode::Enter => {
                if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
                    if !path.is_dir() {
                        self.audio.play(path);
                        self.meta_manager.update_current(FileData::new(), path);
                        self.data = self.meta_manager.current.clone();
                    }
                }
            }

            KeyCode::Char('a') => {
                if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
                    if !path.is_dir() {
                        match self.audio.sink_len() {
                            0 => {
                                self.audio.play(path);
                                self.meta_manager.update_current(FileData::new(), path);
                                self.data = self.meta_manager.current.clone();
                            }
                            _ => {
                                self.audio.append(path);
                                self.meta_manager.queue_metadata(FileData::new(), path);
                            }
                        }
                    }
                }
            }

            KeyCode::Up | KeyCode::Char('k') => self.file_browser.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.file_browser.navigate_down(),
            KeyCode::Left | KeyCode::Char('h') => self.file_browser.navigate_back(),
            KeyCode::Right | KeyCode::Char('l') => self.file_browser.navigate_into(),

            KeyCode::PageUp => self.file_browser.goto_top(),
            KeyCode::PageDown => self.file_browser.goto_bottom(),

            KeyCode::Char('.') | KeyCode::Char('>') => self.audio.adjust_speed(speed_delta),
            KeyCode::Char(',') | KeyCode::Char('<') => self.audio.adjust_speed(speed_delta * -1),
            KeyCode::Char('/') => self.audio.reset_speed(),

            KeyCode::Char('=') | KeyCode::Char('+') => self.audio.adjust_volume(audio_delta),
            KeyCode::Char('-') | KeyCode::Char('_') => self.audio.adjust_volume(audio_delta * -1),
            KeyCode::Char('m') => self.audio.toggle_mute(),

            KeyCode::Char('p') => self.audio.toggle_pause(),

            KeyCode::Char('s') => match self.audio.sink_len() {
                0 => {}
                1 => {
                    self.audio.clear_sink();
                    self.data = self.meta_manager.pop_next().unwrap_or(FileData {
                        raw_file: None,
                        album: None,
                        artist: None,
                        title: None,
                        year: None,
                        duration_display: None,
                        duration_as_secs: None,
                        track_number: None,
                    });
                }
                _ => {
                    self.audio.sink_skip();
                    self.data = self.meta_manager.pop_next().unwrap();
                }
            },

            _ => {}
        }
    }
}
