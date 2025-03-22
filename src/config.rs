use serde::Deserialize;
// use std::fs;

#[derive(Deserialize)]
pub struct Colors {
    pub border: String,
    pub currently_playing: String,
    pub directory_path: String,
    pub filesystem_directory: String,
    pub filesystem_file: String,
    pub highlight_color: String,
    pub muted: String,
    pub paused: String,
    pub playback_speed: String,
    pub separators: String,
    pub volume: String,
}

#[derive(Deserialize)]
pub struct ConfigData {
    pub colors: Colors,
}

/* pub struct Config {
    pub border_color: String,
    pub directory_path_color: String,
    pub playback_speed_color: String,
    pub currently_playing_color: String,
    pub paused_color: String,
    pub muted_color: String,
    pub volume_color: String,
    pub separators_color: String,
    pub filesystem_directory_color: String,
    pub filesystem_file_color: String,
}

impl Config {
    pub fn new(file_path: &str) -> Self {
        let content = fs::read_to_string(file_path).expect("Failed to read config file path");
        let data: ConfigData = toml::from_str(&content).expect("Failed to parse toml");

        Config {
            border_color: data.colors.border,
            directory_path_color: data.colors.directory_path,
            playback_speed_color: data.colors.playback_speed,
            currently_playing_color: data.colors.currently_playing,
            paused_color: data.colors.paused,
            muted_color: data.colors.muted,
            volume_color: data.colors.volume,
            separators_color: data.colors.separators,
            filesystem_directory_color: data.colors.filesystem_directory,
            filesystem_file_color: data.colors.filesystem_file,
        }
    }
} */
