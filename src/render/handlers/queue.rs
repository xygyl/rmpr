use crate::{data::metadata::file_data::FileData, render::tui::app::App};

impl App {
    pub fn handle_play(&mut self) {
        if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
            if !path.is_dir() {
                match self.audio.sink_len() {
                    0 => {
                        self.audio.play(path);
                        self.meta_manager
                            .update_current(FileData::new(), path, true);
                        self.data = self.meta_manager.current.clone();
                        self.path_queue.push(path.clone());
                    }
                    _ => {
                        self.path_queue.insert(0, path.clone());
                        self.audio.play(&self.path_queue[0]);

                        self.audio.clear_sink();
                        for element in self.path_queue.iter().skip(1) {
                            self.audio.append(element);
                        }

                        self.meta_manager
                            .update_current(FileData::new(), path, false);
                        self.data = self.meta_manager.current.clone();
                    }
                }
            }
        }
    }

    pub fn handle_append(&mut self) {
        if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
            if !path.is_dir() {
                match self.audio.sink_len() {
                    0 => {
                        self.audio.play(path);
                        self.meta_manager
                            .update_current(FileData::new(), path, true);
                        self.data = self.meta_manager.current.clone();
                    }
                    _ => {
                        self.audio.append(path);
                        self.meta_manager.queue_metadata(FileData::new(), path);
                    }
                }
            }
            self.path_queue.push(path.clone());
        }
    }

    pub fn handle_skip(&mut self) {
        if self.audio.sink_len() > 0 {
            self.path_queue.remove(0);
            self.audio.clear_sink();

            match self.path_queue.get(0) {
                Some(next_path) => {
                    self.audio.play(next_path);
                    for element in self.path_queue.iter().skip(1) {
                        self.audio.append(element);
                    }
                    self.data = self.meta_manager.pop_next().unwrap_or(FileData::new());
                }
                None => {
                    self.data = FileData::new();
                }
            }
        }
    }
}
