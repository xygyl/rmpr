use crate::{data::metadata::file_metadata::FileMetadata, tui::render::app::App};
use std::{
    collections::{HashMap, VecDeque},
    fs::read_dir,
    path::PathBuf,
    sync::Arc,
};

const PLAYABLE_EXTS: [&str; 3] = ["flac", "mp3", "wav"];

impl App {
    /// Plays the song (or the first song in a directory) and sets up the sinks.
    /// # Examples
    /// ```
    /// sink = [1, 2]
    /// handle_play(3)
    /// sink = [3, 1, 2]
    /// ```
    pub fn handle_play(&mut self) {
        if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
            match path.is_dir() {
                true => {
                    // Build a list of (track_number, Arc<FileMetadata>, PathBuf)
                    let mut metadata_list: Vec<(u16, Arc<FileMetadata>, PathBuf)> = Vec::new();
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
                                if PLAYABLE_EXTS.contains(&ext_str.as_ref()) {
                                    let file_metadata =
                                        if let Some(cached) = self.metadata_cache.get(&file_path) {
                                            cached.clone() // Already an Arc<FileMetadata>
                                        } else {
                                            let mut fm = FileMetadata::new();
                                            fm.get_file_data(&file_path);
                                            let arc_fm = Arc::new(fm);
                                            self.metadata_cache
                                                .insert(file_path.clone(), arc_fm.clone());
                                            arc_fm
                                        };
                                    let track_number = file_metadata.track_number.unwrap_or(0);
                                    metadata_list.push((track_number, file_metadata, file_path));
                                }
                            }
                        }
                    }
                    // Sort by track number.
                    metadata_list.sort_by_key(|(track_number, _, _)| *track_number);
                    if let Some((_, first_metadata, first_path)) = metadata_list.first() {
                        // Clear the queues.
                        self.path_queue.clear();
                        self.metadata_queue.queue.clear();
                        // Only push the file path (not its metadata) for the current track.
                        self.path_queue.push_back(first_path.clone());
                        self.audio.play(first_path);
                        self.metadata_queue.current = first_metadata.clone();
                        self.data = first_metadata.clone(); // assuming self.data is also Arc<FileMetadata>
                        // Append only the subsequent tracks.
                        for (_, file_metadata, file_path) in metadata_list.iter().skip(1) {
                            self.audio.append(file_path);
                            self.metadata_queue.queue.push_back(file_metadata.clone());
                            self.path_queue.push_back(file_path.clone());
                        }
                    }
                }
                false => {
                    // Single file branch.
                    let fm = if let Some(cached) = self.metadata_cache.get(path) {
                        cached.clone()
                    } else {
                        let mut fm = FileMetadata::new();
                        fm.get_file_data(path);
                        let arc_fm = Arc::new(fm);
                        self.metadata_cache.insert(path.clone(), arc_fm.clone());
                        arc_fm
                    };
                    if self.audio.is_empty() {
                        self.audio.play(path);
                        self.metadata_queue.current = fm.clone();
                        self.metadata_queue.queue.clear();
                        self.data = fm.clone();
                        self.path_queue.push_back(path.clone());
                    } else {
                        self.path_queue.push_front(path.clone());
                        self.audio.play(&self.path_queue[0]);
                        self.audio.clear_sink();
                        for element in self.path_queue.iter().skip(1) {
                            self.audio.append(element);
                        }
                        self.metadata_queue.current = fm.clone();
                        if !self.metadata_queue.queue.is_empty() {
                            self.metadata_queue.queue[0] = fm.clone();
                        } else {
                            self.metadata_queue.queue.push_back(fm.clone());
                        }
                        self.data = fm.clone();
                    }
                }
            }
        }
    }

    /// Appends a song (or songs if a directory) to the end of the sink.
    /// # Examples
    /// ```
    /// sink = [1, 2]
    /// handle_append(3)
    /// sink = [1, 2, 3]
    /// ```
    pub fn handle_append(&mut self) {
        if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
            match path.is_dir() {
                true => {
                    let mut metadata_list: Vec<(u16, Arc<FileMetadata>, PathBuf)> = Vec::new();
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
                                if PLAYABLE_EXTS.contains(&ext_str.as_ref()) {
                                    let file_metadata =
                                        if let Some(cached) = self.metadata_cache.get(&file_path) {
                                            cached.clone()
                                        } else {
                                            let mut fm = FileMetadata::new();
                                            fm.get_file_data(&file_path);
                                            let arc_fm = Arc::new(fm);
                                            self.metadata_cache
                                                .insert(file_path.clone(), arc_fm.clone());
                                            arc_fm
                                        };
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
                                self.metadata_queue.current = first_metadata.clone();
                                self.metadata_queue.queue.clear();
                                // Do not add the current track's metadata to the queue.
                                self.data = first_metadata.clone();
                                self.path_queue.clear();
                                self.path_queue.push_back(first_path.clone());
                                for (_, file_metadata, file_path) in metadata_list.iter().skip(1) {
                                    self.audio.append(file_path);
                                    self.metadata_queue.queue.push_back(file_metadata.clone());
                                    self.path_queue.push_back(file_path.clone());
                                }
                            }
                            false => {
                                for (_, file_metadata, file_path) in metadata_list {
                                    self.audio.append(&file_path);
                                    self.metadata_queue.queue.push_back(file_metadata);
                                    self.path_queue.push_back(file_path);
                                }
                            }
                        }
                    }
                }
                false => {
                    let fm = if let Some(cached) = self.metadata_cache.get(path) {
                        cached.clone()
                    } else {
                        let mut fm = FileMetadata::new();
                        fm.get_file_data(path);
                        let arc_fm = Arc::new(fm);
                        self.metadata_cache.insert(path.clone(), arc_fm.clone());
                        arc_fm
                    };
                    match self.audio.is_empty() {
                        true => {
                            self.audio.play(path);
                            self.metadata_queue.current = fm.clone();
                            self.metadata_queue.queue.clear();
                            self.data = fm.clone();
                        }
                        false => {
                            self.audio.append(path);
                            self.metadata_queue.queue.push_back(fm.clone());
                        }
                    }
                    self.path_queue.push_back(path.clone());
                }
            }
        }
    }

    /// Skips the current element in the sink, re-appends the next elements,
    /// and updates the metadata for the new head of the sink.
    pub fn handle_skip(&mut self) {
        if !self.audio.is_empty() {
            // Remove the current file from the path queue.
            self.path_queue.pop_front();
            self.audio.clear_sink();
            match self.path_queue.front() {
                Some(next_path) => {
                    self.audio.play(next_path);
                    for element in self.path_queue.iter().skip(1) {
                        self.audio.append(element);
                    }
                    // Pop the next track's metadata (which should correspond to next_path).
                    if let Some(new_meta) = self.metadata_queue.pop_next() {
                        self.data = new_meta.clone();
                        self.metadata_queue.current = new_meta;
                    } else {
                        self.data = Arc::new(FileMetadata::new());
                    }
                }
                None => {
                    self.data = Arc::new(FileMetadata::new());
                }
            }
        }
    }
}
