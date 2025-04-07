use audiotags::Tag;
use std::path::PathBuf;

/// Encapsulates file data information
#[derive(Clone)]
pub struct FileMetadata {
    pub raw_file: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub duration_display: Option<(f64, f64)>,
    pub duration_as_secs: Option<f64>,
    pub track_number: Option<u16>,
}

impl FileMetadata {
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

    /// Sets FileMetadata with the respective values from the file
    pub fn get_file_data(&mut self, path: &PathBuf) {
        let valid_exts = ["flac", "mp3", "m4a", "mp4"];

        match path.extension() {
            Some(ext) => {
                let file_ext = ext.to_string_lossy().to_ascii_lowercase();
                match valid_exts.contains(&file_ext.as_str()) {
                    true => {
                        let tags = Tag::default().read_from_path(path).unwrap();
                        self.raw_file = path.file_name().map(|n| n.to_string_lossy().to_string());
                        self.album = tags.album_title().map(|n| n.to_string());
                        self.artist = tags.artist().map(|n| n.to_string());
                        self.title = tags.title().map(|n| n.to_string());
                        self.year = tags.year();
                        self.duration_display = tags.duration().map(FileMetadata::sec_to_min_sec);
                        self.duration_as_secs = tags.duration();
                        self.track_number = tags.track_number();
                    }
                    false => {
                        self.raw_file = path.file_name().map(|n| n.to_string_lossy().to_string())
                    }
                }
            }
            None => {}
        }
    }

    /// Display album or nothing
    pub fn display_album(&self) -> String {
        match self.album.as_ref() {
            Some(display) => format!("{}", display),
            None => "".to_string(),
        }
    }

    /// Display artists or nothing
    pub fn display_artist(&self) -> String {
        match self.artist.as_ref() {
            Some(artist) => format!("{}", artist),
            None => "".to_string(),
        }
    }

    /// Display title, or raw file, or nothing if neither is found
    pub fn display_title(&self) -> String {
        match self.title.as_ref() {
            Some(title) => format!("{}", title),
            None => match &self.raw_file {
                Some(raw_file) => format!("{}", raw_file),
                None => "".to_string(),
            },
        }
    }

    /// Display year or nothing
    pub fn display_year(&self) -> String {
        match self.year {
            Some(year) => format!("{}", year),
            None => "".to_string(),
        }
    }

    /// Display track_number or nothing
    pub fn display_track_number(&self) -> String {
        match self.track_number {
            Some(track_number) => format!("{}", track_number),
            None => "".to_string(),
        }
    }

    /// Converts seconds to seconds and minutes
    fn sec_to_min_sec(duration: f64) -> (f64, f64) {
        let min = (duration / 60.0).floor();
        let sec = (duration % 60.0).floor();
        (min, sec)
    }

    /// Display duration_display or nothing
    pub fn display_duration_display(&self) -> String {
        match self.duration_display {
            Some((min, sec)) => format!("{:.0}:{:02.0}", min, sec),
            None => "".to_string(),
        }
    }
}
