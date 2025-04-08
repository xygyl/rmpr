use std::error::Error;

use crate::tui::render::app::run_tui;

mod data;
mod handlers;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    run_tui()
}
