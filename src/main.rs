mod audio;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::run_tui()
}
