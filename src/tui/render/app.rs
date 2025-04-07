use crate::{
    data::{
        config::{ConfigData, load_config},
        metadata::{file_metadata::FileMetadata, metadata_queue::MetadataQueue},
    },
    handlers::input_handler::InputHandler,
    tui::fs_browser::FileBrowser,
};
use crossterm::{
    event::poll,
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use ratatui::DefaultTerminal;
use std::{
    env, io,
    path::PathBuf,
    thread::sleep,
    time::{Duration, Instant},
};

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
    pub config: ConfigData,
    pub meta_manager: MetadataQueue,
    pub file_browser: FileBrowser,
    pub audio: InputHandler,
    pub data: FileMetadata,
    pub path_queue: Vec<PathBuf>,
    pub prog_bar: f64,
    pub exit: bool,
}

impl App {
    pub fn new(initial_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let music_dir = load_config().directories.music_directory;

        let final_dir = match music_dir.exists() {
            true => music_dir,
            false => initial_dir,
        };

        Ok(Self {
            config: load_config(),
            meta_manager: MetadataQueue::new(),
            file_browser: FileBrowser::new(final_dir),
            audio: InputHandler::new()?,
            data: FileMetadata::new(),
            path_queue: Vec::new(),
            prog_bar: f64::MIN_POSITIVE,
            exit: false,
        })
    }

    pub fn update_tui(&mut self) {
        if self.audio.is_empty() {
            self.prog_bar = 0.0;
            return;
        }
        // Displays in milliseconds / milliseconds for higher resolution seekbar
        self.prog_bar = (self.audio.sink_pos_millis() as f64
            / (self.data.duration_as_secs.unwrap() * 1000.0))
            .clamp(0.0, 1.0);
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        let update_interval = Duration::from_millis(100);
        while !self.exit {
            let loop_start = Instant::now();

            while loop_start.elapsed() < update_interval {
                if poll(Duration::from_millis(0))? {
                    self.handle_events()?;
                    break;
                }
                // Sleep to avoid busy waiting
                sleep(Duration::from_millis(1));
            }

            self.file_browser.update_entries()?;
            self.update_tui();
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }
}
