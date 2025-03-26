use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
#[serde(default)]
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
    pub volume: String,
}

#[derive(Deserialize)]
pub struct ConfigData {
    pub colors: Colors,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            border: "#FFFFFF".to_string(),
            currently_playing: "#FFFF00".to_string(),
            directory_path: "#00FF00".to_string(),
            filesystem_directory: "#598EFF".to_string(),
            filesystem_file: "#FFFFFF".to_string(),
            highlight_color: "#FF0000".to_string(),
            muted: "#FF0000".to_string(),
            paused: "#00FF00".to_string(),
            playback_speed: "#598EFF".to_string(),
            volume: "#598EFF".to_string(),
        }
    }
}

pub fn load_config() -> ConfigData {
    let config_path = dirs::home_dir()
        .map(|mut path| {
            path.push(".config/rmpr/config.toml");
            path
        })
        .expect("Could not find home directory");

    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read config file at {}", config_path.display()));

    toml::from_str(&config_content).unwrap_or_else(|_| panic!("Failed to parse TOML config"))
}
