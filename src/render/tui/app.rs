use crate::{
    data::{
        config::{load_config, ConfigData},
        metadata::{file_data::FileData, metadata_manager::MetadataQueue},
    },
    handlers::input::InputHandler,
    render::browser::FileBrowser,
};
use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use ratatui::DefaultTerminal;
use std::{env, io, path::PathBuf, time::Duration};

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
    pub data: FileData,
    pub path_queue: Vec<PathBuf>,
    pub seekbar: f64,
    pub exit: bool,
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
            path_queue: Vec::new(),
            seekbar: f64::MIN_POSITIVE,
            exit: false,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let timeout = Duration::from_millis(1);
        while !self.exit {
            self.file_browser.update_entries()?;
            if event::poll(timeout)? {
                self.handle_events()?;
            }
            self.update();
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }
}
