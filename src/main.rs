mod browser;
mod config;
mod file_data;
mod input_handler;
mod sink_handler;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::run_tui()
}
