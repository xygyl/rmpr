mod audio;
mod audio_playing;
mod browser;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::run_tui()
}
