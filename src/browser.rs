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
        let mut files = Vec::new();

        for entry in fs::read_dir(&self.current_dir)? {
            if let Ok(entry) = entry {
                let path = entry.path();
                // Skip hidden files
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }
                if path.is_dir() {
                    directories.push(path);
                } else {
                    files.push(path);
                }
            }
        }
        directories.sort();
        files.sort();
        self.entries = directories.into_iter().chain(files.into_iter()).collect();
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

    pub fn list_items(&self) -> Vec<ListItem> {
        self.entries
            .iter()
            .map(|entry| {
                let file_name = entry
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| String::from("Unknown"));
                let style = if entry.is_dir() {
                    Style::default().fg(Color::from_str("#6B5DFF").unwrap())
                } else {
                    Style::default().fg(Color::from_str("#F98771").unwrap())
                };
                ListItem::new(file_name).style(style)
            })
            .collect()
    }
}
