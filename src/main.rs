mod audio_playing;
mod browser;
mod sink_handling;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::run_tui()
}
