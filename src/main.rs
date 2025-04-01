use crate::render::tui::app::run_tui;

mod data;
mod handlers;
mod render;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_tui()
}
