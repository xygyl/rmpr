use crate::{config::load_config, file_data::FileData};
use ratatui::{
    style::{Color, Style},
    widgets::{ListItem, ListState},
};
use std::{collections::HashMap, fs, io, path::PathBuf, str::FromStr};

/// Encapsulates file system browsing state and behavior
pub struct FileBrowser {
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
            current_dir: initial_dir,
            selected: 0,
            list_state,
            sel_map,
            entries: Vec::new(),
        }
    }

    /// Refreshes the list of entries from the current directory
    pub fn update_entries(&mut self) -> io::Result<()> {
        let mut directories = Vec::new();
        let mut metadata_list = Vec::new();

        let playable_exts = ["flac", "mp3"];

        for entry in fs::read_dir(&self.current_dir)? {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }

                if path.is_dir() {
                    directories.push(path);
                } else if let Some(ext) = path.extension() {
                    if playable_exts.contains(&ext.to_string_lossy().to_ascii_lowercase().as_ref())
                    {
                        let mut file_data = FileData::new();
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

        directories.sort();
        metadata_list.sort_by_key(|&(track_number, _, _)| track_number);

        let playable_files: Vec<PathBuf> =
            metadata_list.into_iter().map(|(_, _, path)| path).collect();

        self.entries = directories
            .into_iter()
            .chain(playable_files.into_iter())
            .collect();

        self.list_state.select(if self.entries.is_empty() {
            None
        } else {
            Some(self.selected)
        });

        Ok(())
    }

    pub fn navigate_up(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.entries.len() - 1;
        } else {
            self.selected -= 1;
        }
        self.sel_map.insert(self.current_dir.clone(), self.selected);
    }

    pub fn navigate_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        if self.selected < self.entries.len() - 1 {
            self.selected += 1;
        } else {
            self.selected = 0;
        }
        self.sel_map.insert(self.current_dir.clone(), self.selected);
    }

    pub fn navigate_into(&mut self) {
        if let Some(path) = self.entries.get(self.selected) {
            if path.is_dir() {
                self.current_dir = path.clone();
                self.selected = *self.sel_map.get(&self.current_dir).unwrap_or(&0);
            }
        }
    }

    pub fn navigate_back(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected = *self.sel_map.get(&self.current_dir).unwrap_or(&0);
        }
    }

    pub fn goto_top(&mut self) {
        self.selected = 0
    }

    pub fn goto_bottom(&mut self) {
        self.selected = self.entries.len();
    }

    pub fn list_items(&self) -> Vec<ListItem> {
        let config_data = load_config();

        let filesystem_directory = config_data.colors.filesystem_directory.as_str();
        let filesystem_file = config_data.colors.filesystem_file.as_str();

        self.entries
            .iter()
            .map(|entry| {
                let display_name = if entry.is_dir() {
                    entry
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Unknown".to_string())
                } else {
                    let mut file_data = FileData::new();
                    file_data.get_file_data(entry);
                    file_data
                        .title
                        .unwrap_or(file_data.raw_file.unwrap_or("Unknown".to_string()))
                };

                let style = if entry.is_dir() {
                    Style::default().fg(Color::from_str(filesystem_directory).unwrap())
                } else {
                    Style::default().fg(Color::from_str(filesystem_file).unwrap())
                };

                ListItem::new(display_name).style(style)
            })
            .collect()
    }
}
