use crate::file_data::FileData;
use std::path::PathBuf;

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
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}
