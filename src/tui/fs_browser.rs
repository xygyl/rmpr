use crate::data::{
    config::{ConfigData, load_config},
    metadata::file_metadata::FileMetadata,
};
use ratatui::{
    style::{Color, Style},
    widgets::{ListItem, ListState},
};
use std::{collections::HashMap, fs::read_dir, io, path::PathBuf, str::FromStr};

/// Encapsulates file system browsing state and behavior.
pub struct FileBrowser {
    pub config: ConfigData,
    pub current_dir: PathBuf,
    pub selected: usize,
    pub list_state: ListState,
    pub sel_map: HashMap<PathBuf, usize>,
    pub entries: Vec<PathBuf>,
}

impl FileBrowser {
    pub fn new(initial_dir: PathBuf) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let mut sel_map = HashMap::new();
        sel_map.insert(initial_dir.clone(), 0);
        Self {
            config: load_config(),
            current_dir: initial_dir,
            selected: 0,
            list_state,
            sel_map,
            entries: Vec::new(),
        }
    }

    /// Refreshes the list of entries from the current directory.
    pub fn update_entries(&mut self) -> io::Result<()> {
        let mut directories = Vec::new();
        let mut metadata_list = Vec::new();

        let playable_exts = ["flac", "mp3", "wav"];

        for entry in read_dir(&self.current_dir)? {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }

                match path.is_dir() {
                    true => directories.push(path),
                    false => {
                        if let Some(ext) = path.extension() {
                            if playable_exts
                                .contains(&ext.to_string_lossy().to_ascii_lowercase().as_ref())
                            {
                                let mut file_data = FileMetadata::new();
                                file_data.get_file_data(&path);
                                let track_number = file_data.track_number.unwrap_or(0);
                                let title = file_data
                                    .title
                                    .unwrap_or_else(|| path.to_string_lossy().to_string());

                                metadata_list.push((track_number, title, path));
                            }
                        }
                    }
                }
            }
        }

        directories.sort();
        metadata_list.sort_by_key(|&(track_number, _, _)| track_number);

        let playable_files: Vec<PathBuf> =
            metadata_list.into_iter().map(|(_, _, path)| path).collect();

        self.entries = directories
            .into_iter()
            .chain(playable_files.into_iter())
            .collect();

        self.list_state.select(match self.entries.is_empty() {
            true => None,
            false => Some(self.selected),
        });

        Ok(())
    }

    /// Moves the cursor up one element or goes to the bottom if at the top.
    pub fn navigate_up(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        match self.selected {
            0 => self.selected = self.entries.len() - 1,
            _ => self.selected -= 1,
        }
        self.sel_map.insert(self.current_dir.clone(), self.selected);
    }

    /// Moves the cursor down one element or goes to the top if at the bottom.
    pub fn navigate_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        match self.selected < self.entries.len() - 1 {
            true => self.selected += 1,
            false => self.selected = 0,
        }
        self.sel_map.insert(self.current_dir.clone(), self.selected);
    }

    /// Navigates into the selected directory, either setting the cursor to the saved position or 0.
    pub fn navigate_into(&mut self) {
        if let Some(path) = self.entries.get(self.selected) {
            if path.is_dir() {
                self.current_dir = path.clone();
                self.selected = *self.sel_map.get(&self.current_dir).unwrap_or(&0);
            }
        }
    }

    /// Navigates into the previous directory, either setting the cursor to the saved position or 0.
    pub fn navigate_back(&mut self) {
        if self.current_dir == self.config.directories.music_directory {
            return;
        }
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected = *self.sel_map.get(&self.current_dir).unwrap_or(&0);
        }
    }

    /// Moves the cursor to the top of the list.
    pub fn goto_top(&mut self) {
        self.selected = 0
    }

    /// Moves the cursor to the bottom of the list.
    pub fn goto_bottom(&mut self) {
        self.selected = self.entries.len();
    }

    /// Navigates to the user's set music directory and sets selected to the top (1).
    pub fn goto_music_dir(&mut self) {
        if self.current_dir == self.config.directories.music_directory {
            return;
        }
        self.current_dir = self.config.directories.music_directory.clone();
        self.goto_top();
    }

    /// Lists all items in the directory; displays directories as their name, files as their metadata name, and both by their respective colors.
    pub fn list_items(&self) -> Vec<ListItem> {
        let fs_directory = &self.config.colors.fs_directory;
        let fs_file = &self.config.colors.fs_file;

        self.entries
            .iter()
            .map(|entry| {
                let display_name = match entry.is_dir() {
                    true => entry
                        .file_name()
                        .map(|s| format!("[{}]", s.to_string_lossy().to_string()))
                        .unwrap_or_else(|| "Unknown".to_string()),
                    false => {
                        let mut file_data = FileMetadata::new();
                        file_data.get_file_data(entry);
                        file_data
                            .title
                            .unwrap_or(file_data.raw_file.unwrap_or("Unknown".to_string()))
                    }
                };

                let style = match entry.is_dir() {
                    true => Style::default().fg(Color::from_str(fs_directory).unwrap()),
                    false => Style::default().fg(Color::from_str(fs_file).unwrap()),
                };

                ListItem::new(display_name).style(style)
            })
            .collect()
    }
}
