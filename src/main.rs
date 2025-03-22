// use serde::Deserialize;
use std::{env, fs};
use toml;

use crate::config::ConfigData;

mod audio_playing;
mod browser;
mod config;
mod sink_handling;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME").expect("Couldn't find home directory");
    let config_path = format!("{}/.config/rmpr/config.toml", home_dir);

    let config = fs::read_to_string(&config_path)
        .expect(&format!("Failed to read config file at {}", config_path));
    let config_data: ConfigData = toml::from_str(&config).expect("Failed to parse TOML config");

    println!("{}", config_data.colors.border);
    println!("{}", config_data.colors.directory_path);
    println!("{}", config_data.colors.playback_speed);
    println!("{}", config_data.colors.currently_playing);
    println!("{}", config_data.colors.paused);
    println!("{}", config_data.colors.muted);
    println!("{}", config_data.colors.volume);
    println!("{}", config_data.colors.separators);
    println!("{}", config_data.colors.filesystem_directory);
    println!("{}", config_data.colors.filesystem_file);

    tui::run_tui()
}
