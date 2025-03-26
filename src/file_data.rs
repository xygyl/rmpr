use audiotags::Tag;
use std::path::PathBuf;

/// Encapsulates file data information
pub struct FileData {
    pub raw_file: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub duration: Option<f64>,
    pub track_number: Option<u16>,
}

impl FileData {
    pub fn new() -> Self {
        Self {
            raw_file: None,
            album: None,
            artist: None,
            title: None,
            year: None,
            duration: None,
            track_number: None,
        }
    }

    pub fn get_file_data(&mut self, path: &PathBuf) {
        let tags = Tag::default().read_from_path(path).unwrap();

        self.raw_file = path.file_name().map(|n| n.to_string_lossy().to_string());

        self.album = tags.album_title().map(|n| n.to_string());
        self.artist = tags.artist().map(|n| n.to_string());
        self.title = tags.title().map(|n| n.to_string());
        self.year = tags.year();
        self.duration = tags.duration();
        self.track_number = tags.track_number();
    }
}
