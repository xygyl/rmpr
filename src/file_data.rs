use audiotags::Tag;
use std::path::PathBuf;

/// Encapsulates file data information
pub struct FileData {
    pub raw_file: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub duration_display: Option<(f64, f64)>,
    pub duration_as_secs: Option<f64>,
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
            duration_display: None,
            duration_as_secs: None,
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
        self.duration_display = tags.duration().map(|d| FileData::sec_to_min_sec(d));
        self.duration_as_secs = tags.duration();
        self.track_number = tags.track_number();
    }

    fn sec_to_min_sec(duration: f64) -> (f64, f64) {
        let min = (duration / 60.0).floor();
        let sec = (duration % 60.0).floor();
        (min, sec)
    }

    pub fn duration_as_string(&self) -> String {
        match self.duration_display {
            Some((min, sec)) => format!("{:.0}:{:02.0}", min, sec),
            None => "".to_string(),
        }
    }
}
