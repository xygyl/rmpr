use std::{fs::read_dir, path::PathBuf};

use crate::{data::metadata::file_metadata::FileMetadata, tui::render::app::App};

impl App {
    /// Creates a sink and appends audio if the sink is empty or non-existent.
    /// Plays the audio and appends the current sink elements if the sink isn't empty.
    /// # Examples
    /// ```
    /// sink = [1, 2]
    /// handle_play(3)
    /// sink = [3, 1, 2]
    /// ```
    pub fn handle_play(&mut self) {
        let playable_exts = ["flac", "mp3", "wav"];
        if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
            match path.is_dir() {
                true => {
                    let mut metadata_list: Vec<(u16, FileMetadata, PathBuf)> = Vec::new();

                    if let Ok(entries) = read_dir(path) {
                        for entry in entries.filter_map(|e| e.ok()) {
                            let file_path = entry.path();

                            if let Some(file_name) = file_path.file_name() {
                                if file_name.to_string_lossy().starts_with('.') {
                                    continue;
                                }
                            }
                            if let Some(ext) = file_path.extension() {
                                let ext_str = ext.to_string_lossy().to_ascii_lowercase();
                                if playable_exts.contains(&ext_str.as_ref()) {
                                    let mut file_metadata = FileMetadata::new();
                                    file_metadata.get_file_data(&file_path);
                                    let track_number = file_metadata.track_number.unwrap_or(0);
                                    metadata_list.push((track_number, file_metadata, file_path));
                                }
                            }
                        }
                    }
                    metadata_list.sort_by_key(|(track_number, _, _)| *track_number);

                    if let Some((_, first_metadata, first_path)) = metadata_list.first() {
                        self.path_queue.clear();
                        self.meta_manager.queue.clear();
                        self.path_queue.push(first_path.clone());
                        self.meta_manager.queue.push(first_metadata.clone());
                        self.audio.play(first_path);
                        self.meta_manager.current = first_metadata.clone();
                        self.data = first_metadata.clone();
                        for (_, file_metadata, file_path) in metadata_list.iter().skip(1) {
                            self.audio.append(file_path);
                            self.meta_manager.queue.push(file_metadata.clone());
                            self.path_queue.push(file_path.clone());
                        }
                    }
                }
                false => match self.audio.is_empty() {
                    true => {
                        self.audio.play(path);
                        let mut fm = FileMetadata::new();
                        fm.get_file_data(path);
                        self.meta_manager.current = fm.clone();
                        self.meta_manager.queue.clear();
                        self.meta_manager.queue.push(fm.clone());
                        self.data = fm;
                        self.path_queue.push(path.clone());
                    }
                    false => {
                        self.path_queue.insert(0, path.clone());
                        self.audio.play(&self.path_queue[0]);
                        self.audio.clear_sink();
                        for element in self.path_queue.iter().skip(1) {
                            self.audio.append(element);
                        }
                        let mut fm = FileMetadata::new();
                        fm.get_file_data(path);
                        self.meta_manager.current = fm.clone();
                        if !self.meta_manager.queue.is_empty() {
                            self.meta_manager.queue[0] = fm.clone();
                        } else {
                            self.meta_manager.queue.push(fm.clone());
                        }
                        self.data = fm;
                    }
                },
            }
        }
    }

    /// Creates sink if it's empty (equivalent to handle_play).
    /// Appends song to the end of the sink if it isn't empty.
    /// # Examples
    /// ```
    /// sink = [1, 2]
    /// handle_append(3)
    /// sink = [1, 2, 3]
    /// ```
    pub fn handle_append(&mut self) {
        let playable_exts = ["flac", "mp3", "wav"];
        if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
            match path.is_dir() {
                true => {
                    let mut metadata_list: Vec<(u16, FileMetadata, PathBuf)> = Vec::new();
                    if let Ok(entries) = read_dir(path) {
                        for entry in entries.filter_map(|e| e.ok()) {
                            let file_path = entry.path();
                            if let Some(file_name) = file_path.file_name() {
                                if file_name.to_string_lossy().starts_with('.') {
                                    continue;
                                }
                            }
                            if let Some(ext) = file_path.extension() {
                                let ext_str = ext.to_string_lossy().to_ascii_lowercase();
                                if playable_exts.contains(&ext_str.as_ref()) {
                                    let mut file_metadata = FileMetadata::new();
                                    file_metadata.get_file_data(&file_path);
                                    let track_number = file_metadata.track_number.unwrap_or(0);
                                    metadata_list.push((track_number, file_metadata, file_path));
                                }
                            }
                        }
                    }
                    metadata_list.sort_by_key(|(track_number, _, _)| *track_number);
                    if !metadata_list.is_empty() {
                        match self.audio.is_empty() {
                            true => {
                                let (_, ref first_metadata, ref first_path) = metadata_list[0];
                                self.audio.play(first_path);
                                self.meta_manager.current = first_metadata.clone();
                                self.meta_manager.queue.clear();
                                self.meta_manager.queue.push(first_metadata.clone());
                                self.data = first_metadata.clone();
                                self.path_queue.clear();
                                self.path_queue.push(first_path.clone());
                                for (_, file_metadata, file_path) in metadata_list.iter().skip(1) {
                                    self.audio.append(file_path);
                                    self.meta_manager.queue.push(file_metadata.clone());
                                    self.path_queue.push(file_path.clone());
                                }
                            }
                            false => {
                                for (_, file_metadata, file_path) in metadata_list {
                                    self.audio.append(&file_path);
                                    self.meta_manager.queue.push(file_metadata);
                                    self.path_queue.push(file_path);
                                }
                            }
                        }
                    }
                }
                false => {
                    match self.audio.is_empty() {
                        true => {
                            self.audio.play(path);
                            let mut fm = FileMetadata::new();
                            fm.get_file_data(path);
                            self.meta_manager.current = fm.clone();
                            self.meta_manager.queue.clear();
                            self.meta_manager.queue.push(fm.clone());
                            self.data = fm;
                        }
                        false => {
                            self.audio.append(path);
                            let mut fm = FileMetadata::new();
                            fm.get_file_data(path);
                            self.meta_manager.queue.push(fm);
                        }
                    }
                    self.path_queue.push(path.clone());
                }
            }
        }
    }
    /// Skips the current element in the sink, re-appends the next elements,
    /// and updates the metadata for the new head of the sink.
    pub fn handle_skip(&mut self) {
        if self.audio.get_len() > 0 {
            // Remove the current file from the queue.
            self.path_queue.remove(0);
            self.audio.clear_sink();

            match self.path_queue.get(0) {
                Some(next_path) => {
                    self.audio.play(next_path);
                    for element in self.path_queue.iter().skip(1) {
                        self.audio.append(element);
                    }
                    // Pop the corresponding metadata (which now lines up with the queue).
                    self.data = self.meta_manager.pop_next().unwrap_or(FileMetadata::new());
                }
                None => {
                    self.data = FileMetadata::new();
                }
            }
        }
    }
}
