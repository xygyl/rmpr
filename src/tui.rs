use crate::browser::FileBrowser;
use crate::{audio_playing::AudioPlaying, config::load_config};
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
    file_browser: FileBrowser,
    audio: AudioPlaying,
    exit: bool,
}

impl App {
    pub fn new(initial_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Checks if the user has a home dir. If so, go to ~/Music.
        // If not, go to current directory
        // This code exists because dirs::audio_dir() returned None
        let home_dir = dirs::home_dir().map(|mut path| {
            path.push("Music");
            path
        });
        let final_dir = if let Some(ref path) = home_dir {
            if path.exists() {
                path.clone()
            } else {
                initial_dir
            }
        } else {
            initial_dir
        };

        Ok(Self {
            file_browser: FileBrowser::new(final_dir),
            audio: AudioPlaying::new()?,
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
        let config_data = load_config();

        let border = config_data.colors.border;
        let currently_playing = config_data.colors.currently_playing;
        let directory_path = config_data.colors.directory_path;
        let highlight_color = config_data.colors.highlight_color;
        let muted = config_data.colors.muted;
        let paused = config_data.colors.paused;
        let playback_speed = config_data.colors.playback_speed;
        let volume = config_data.colors.volume;

        let current_dir = self.file_browser.current_dir.to_string_lossy().to_string();

        // Displays HOME as ~ instead of /home/user
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
                format!(
                    "{}",
                    self.audio
                        .title
                        .as_deref()
                        .or_else(|| self.audio.raw_file.as_deref())
                        .unwrap_or("")
                ),
                Style::default().fg(Color::from_str(&currently_playing).unwrap()),
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
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

            KeyCode::Enter => {
                if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
                    if !path.is_dir() {
                        self.audio.play(path);
                    }
                }
            }

            KeyCode::Up | KeyCode::Char('k') => self.file_browser.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.file_browser.navigate_down(),
            KeyCode::Left | KeyCode::Char('h') => self.file_browser.navigate_back(),
            KeyCode::Right | KeyCode::Char('l') => self.file_browser.navigate_into(),

            KeyCode::Char('.') | KeyCode::Char('>') => self.audio.adjust_speed(25),
            KeyCode::Char(',') | KeyCode::Char('<') => self.audio.adjust_speed(-25),
            KeyCode::Char('/') => self.audio.reset_speed(),

            KeyCode::Char('=') | KeyCode::Char('+') => self.audio.adjust_volume(2),
            KeyCode::Char('-') | KeyCode::Char('_') => self.audio.adjust_volume(-2),
            KeyCode::Char('m') => self.audio.toggle_mute(),

            KeyCode::Char('p') => self.audio.toggle_pause(),

            _ => {}
        }
    }
}
