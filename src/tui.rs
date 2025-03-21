use crate::audio_playing::AudioPlaying;
use crate::browser::FileBrowser;
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
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
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
        Ok(Self {
            file_browser: FileBrowser::new(initial_dir),
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
        let size = frame.area();

        let bottom_line = Line::from(vec![
            Span::styled(
                format!("Paused: {:>5}", self.audio.paused),
                Style::default().fg(Color::from_str("#417BFF").unwrap()),
            ),
            Span::styled(
                " | ",
                Style::default().fg(Color::from_str("#00FFAA").unwrap()),
            ),
            Span::styled(
                format!("Muted: {:>5}", self.audio.muted),
                Style::default().fg(Color::from_str("#AE5DFF").unwrap()),
            ),
            Span::styled(
                " | ",
                Style::default().fg(Color::from_str("#00FFAA").unwrap()),
            ),
            Span::styled(
                format!("Volume: {:>3.2}%", self.audio.vol),
                Style::default().fg(Color::from_str("#FF5D85").unwrap()),
            ),
        ]);

        let block = Block::bordered()
            .border_style(Style::default().fg(Color::from_str("#1F153E").unwrap()))
            .border_set(border::THICK)
            .title_top(
                Line::from(Span::styled(
                    format!("{}", self.file_browser.current_dir.display()),
                    Style::default().fg(Color::from_str("#00FFAA").unwrap()),
                ))
                .left_aligned(),
            )
            .title_top(
                Line::from(Span::styled(
                    format!(
                        "Playback speed: x{:<4}",
                        (self.audio.play_speed as f32) / 100.0
                    ),
                    Style::default().fg(Color::from_str("#FF5DC8").unwrap()),
                ))
                .right_aligned(),
            )
            .title_bottom(
                Line::from(match &self.audio.playing_file {
                    Some(file) => Span::styled(
                        format!("Playing: {}", file),
                        Style::default().fg(Color::from_str("#F1FF5D").unwrap()),
                    ),
                    None => Span::raw(""),
                })
                .left_aligned(),
            )
            .title_bottom(bottom_line.right_aligned());

        let items = self.file_browser.list_items();
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().fg(Color::from_str("#00EAFF").unwrap()));

        frame.render_stateful_widget(list, size, &mut self.file_browser.list_state.clone());
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
            KeyCode::Char(',') | KeyCode::Char('<') => self.audio.adjust_speed(-25),
            KeyCode::Char('.') | KeyCode::Char('>') => self.audio.adjust_speed(25),
            KeyCode::Char('/') => self.audio.reset_speed(),
            KeyCode::Char('m') => self.audio.toggle_mute(),
            KeyCode::Char('p') => self.audio.toggle_pause(),
            KeyCode::Char('-') | KeyCode::Char('_') => self.audio.adjust_volume(-5),
            KeyCode::Char('=') | KeyCode::Char('+') => self.audio.adjust_volume(5),
            _ => {}
        }
    }
}
