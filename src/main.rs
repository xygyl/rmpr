mod browser;
mod config;
mod file_data;
mod input_handling;
mod sink_handling;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::run_tui()
}
