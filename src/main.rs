use crate::tui::tui::run_tui;

mod data;
mod handlers;
mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_tui()
}
