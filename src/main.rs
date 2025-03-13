use std::io::{self, BufReader};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::{env, fs};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;

fn play_flac_file(path: &PathBuf) {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Create an audio output stream.
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to get default audio output stream");

    // Create a sink (for controlling playback).
    let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

    // Decode the FLAC file.
    let source = Decoder::new(reader).expect("Failed to decode FLAC file");
    sink.append(source);

    // Wait until the file finishes playing.
    sink.sleep_until_end();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal in raw mode and enter alternate screen.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Start at the current directory.
    let mut current_dir = env::current_dir()?;
    let mut selected: usize = 0;

    // Create a ListState to track selection.
    let mut list_state = ListState::default();
    list_state.select(Some(selected));

    loop {
        // Read the entries in the current directory.
        let mut entries: Vec<PathBuf> = fs::read_dir(&current_dir)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();
        // Sort entries alphabetically.
        entries.sort();

        // Convert entries to ListItem widgets.
        let items: Vec<ListItem> = entries
            .iter()
            .map(|entry| {
                let file_name = entry
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| String::from("Unknown"));
                // Add a visual hint for directories.
                let display = if entry.is_dir() {
                    format!("{}/", file_name)
                } else {
                    file_name
                };
                ListItem::new(display)
            })
            .collect();

        // Update the ListState with the current selection.
        if selected >= entries.len() && !entries.is_empty() {
            selected = entries.len() - 1;
        }
        list_state.select(if entries.is_empty() {
            None
        } else {
            Some(selected)
        });

        // Draw the UI.
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .borders(Borders::ALL)
                .title(format!("Directory: {}", current_dir.display()));

            // Create a List widget.
            let list = List::new(items)
                .block(block)
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        // Handle key events.
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break, // Quit
                    KeyCode::Down => {
                        if selected < entries.len().saturating_sub(1) {
                            selected += 1;
                        }
                    }
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Left => {
                        if let Some(parent) = current_dir.parent() {
                            current_dir = parent.to_path_buf();
                            selected = 0;
                        }
                    }
                    KeyCode::Enter => {
                        // If no entry is selected, do nothing.
                        if let Some(path) = entries.get(selected) {
                            if let Some(ext) = path.extension() {
                                if ext.to_string_lossy().to_lowercase() == "flac" {
                                    // Spawn a new thread to play the FLAC file.
                                    let path_clone = path.clone();
                                    thread::spawn(move || {
                                        play_flac_file(&path_clone);
                                    });
                                }
                            }
                        }
                    }
                    KeyCode::Right => {
                        if let Some(path) = entries.get(selected) {
                            if path.is_dir() {
                                // Enter directory.
                                current_dir = path.clone();
                                selected = 0;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal state.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
