use crate::data::metadata::file_metadata::FileMetadata;
use std::{collections::VecDeque, path::PathBuf, sync::Arc};

pub struct MetadataQueue {
    pub current: Arc<FileMetadata>,
    pub queue: VecDeque<Arc<FileMetadata>>,
}

impl MetadataQueue {
    pub fn new() -> Self {
        Self {
            current: Arc::new(FileMetadata::new()),
            queue: VecDeque::new(),
        }
    }

    pub fn update_current(&mut self, mut data: FileMetadata, path: &PathBuf, clear: bool) {
        data.get_file_data(path);
        let arc_data = Arc::new(data);
        if clear {
            self.queue.clear();
        }
        // Optionally, if you want to update the queue with the current track,
        // but if you want to keep the current track separate, don’t push it to the queue.
        self.current = arc_data;
    }

    pub fn queue_metadata(&mut self, mut data: FileMetadata, path: &PathBuf) {
        data.get_file_data(path);
        self.queue.push_back(Arc::new(data));
    }

    /// Pops the next metadata (from the front) and makes it current.
    pub fn pop_next(&mut self) -> Option<Arc<FileMetadata>> {
        self.queue.pop_front().map(|next| {
            self.current = next.clone();
            next
        })
    }
}
