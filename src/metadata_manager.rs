use crate::file_data::FileData;
use std::path::PathBuf;

/// Encapsulates metadata queue information for correct displaying
#[derive(Clone)]
pub struct MetadataQueue {
    pub current: FileData,
    pub queue: Vec<FileData>,
}

impl MetadataQueue {
    pub fn new() -> Self {
        Self {
            current: FileData::new(),
            queue: Vec::new(),
        }
    }

    /// Update the current metadata
    pub fn update_current(&mut self, mut data: FileData, path: &PathBuf) {
        data.get_file_data(path);
        self.queue.insert(0, data.clone());
        self.current = data;
    }

    /// Add metadata for an appended file to the queue
    pub fn queue_metadata(&mut self, mut data: FileData, path: &PathBuf) {
        data.get_file_data(path);
        self.queue.push(data);
    }

    /// Pop the next metadata from the queue if available
    pub fn pop_next(&mut self) -> Option<FileData> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(1))
        } else {
            None
        }
    }

    pub fn change_current(&mut self) {
        let mut new_queue: Vec<FileData> = Vec::new();
        new_queue.insert(0, self.current.clone());
        for item in self.queue.clone() {
            new_queue.push(item);
        }
        self.queue = new_queue.clone();
    }
}
