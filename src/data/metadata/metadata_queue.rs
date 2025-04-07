use crate::data::metadata::file_metadata::FileMetadata;
use std::path::PathBuf;

/// Encapsulates metadata queue information for correct displaying
#[derive(Clone)]
pub struct MetadataQueue {
    pub current: FileMetadata,
    pub queue: Vec<FileMetadata>,
}

impl MetadataQueue {
    pub fn new() -> Self {
        Self {
            current: FileMetadata::new(),
            queue: Vec::new(),
        }
    }

    /// Updates the current metadata
    pub fn update_current(&mut self, mut data: FileMetadata, path: &PathBuf, clear: bool) {
        data.get_file_data(path);
        if clear {
            self.queue.clear();
        }
        self.queue.insert(0, data.clone());
        self.current = data;
    }

    /// Appends metadata for a queued song
    pub fn queue_metadata(&mut self, mut data: FileMetadata, path: &PathBuf) {
        data.get_file_data(path);
        self.queue.push(data);
    }

    /// When skipping, remove the current metadata (index 0), set it to the next in the vec, then update current
    pub fn pop_next(&mut self) -> Option<FileMetadata> {
        if !self.queue.is_empty() {
            self.queue.remove(0);
        }
        match self.queue.first() {
            Some(next) => {
                self.current = next.clone();
                Some(next.clone())
            }
            None => {
                self.current = FileMetadata::new();
                None
            }
        }
    }
}
