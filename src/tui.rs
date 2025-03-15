use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use rodio::OutputStream;
use std::{
    collections::HashMap, env, fs, io, path::PathBuf, str::FromStr, sync::Arc, thread,
    time::Duration,
};

use crate::audio::{play_flac_file, set_play_speed, toggle_play_pause, SharedSink};

/// Runs the file browser and TUI event loop.
pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_dir = env::current_dir()?;
    let mut selected: usize = 0;
    let mut list_state = ListState::default();
    list_state.select(Some(selected));
    let mut sel_map: HashMap<PathBuf, usize> = HashMap::new();
    sel_map.insert(current_dir.clone(), 0);

    // Shared output stream and sink for audio control
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink: SharedSink = Arc::new(std::sync::Mutex::new(None));

    loop {
        let mut directories: Vec<PathBuf> = Vec::new();
        let mut flac_files: Vec<PathBuf> = Vec::new();
        let mut other_files: Vec<PathBuf> = Vec::new();

        for entry in fs::read_dir(&current_dir)? {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }
                if path.is_dir() {
                    directories.push(path);
                } else if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().eq_ignore_ascii_case("flac") {
                        flac_files.push(path);
                    } else {
                        other_files.push(path);
                    }
                }
            }
        }

        directories.sort();
        flac_files.sort();
        other_files.sort();

        let entries: Vec<PathBuf> = directories
            .into_iter()
            .chain(flac_files.into_iter())
            .chain(other_files.into_iter())
            .collect();

        let items: Vec<ListItem> = entries
            .iter()
            .map(|entry| {
                let file_name = entry
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| String::from("Unknown"));

                let is_dir = entry.is_dir();
                let display = if is_dir {
                    format!("{}/", file_name)
                } else {
                    file_name
                };

                let style = if is_dir {
                    Style::default().fg(Color::from_str("#6B5DFF").unwrap())
                } else {
                    Style::default().fg(Color::from_str("#F98771").unwrap())
                };

                ListItem::new(display).style(style)
            })
            .collect();

        list_state.select(if entries.is_empty() {
            None
        } else {
            Some(selected)
        });

        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::from_str("#1F153E").unwrap()))
                .title(format!("{}", current_dir.display()))
                .title_position(ratatui::widgets::block::Position::Top)
                .title_style(Style::default().fg(Color::from_str("#00FFAA").unwrap()));

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default().fg(Color::from_str("#00EAFF").unwrap()));
            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        if selected < entries.len() - 1 {
                            selected += 1;
                        } else {
                            selected = 0;
                        }
                        sel_map.insert(current_dir.clone(), selected);
                    }
                    KeyCode::Up => {
                        if selected == 0 {
                            selected = entries.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        sel_map.insert(current_dir.clone(), selected);
                    }
                    KeyCode::Enter => {
                        if let Some(path) = entries.get(selected) {
                            if let Some(ext) = path.extension() {
                                if ext.to_string_lossy() == "flac" {
                                    let stream_handle_clone = stream_handle.clone();
                                    let sink_clone = Arc::clone(&sink);
                                    let path_clone = path.clone();
                                    thread::spawn(move || {
                                        play_flac_file(path_clone, stream_handle_clone, sink_clone);
                                    });
                                }
                            }
                        }
                    }
                    KeyCode::Right => {
                        if let Some(path) = entries.get(selected) {
                            if path.is_dir() {
                                selected = *sel_map.get(path).unwrap_or(&0);
                                current_dir = path.clone();
                            }
                        }
                    }
                    KeyCode::Left => {
                        if let Some(parent) = current_dir.parent() {
                            selected = *sel_map.get(parent).unwrap_or(&0);
                            current_dir = parent.to_path_buf();
                        }
                    }
                    KeyCode::Char(' ') => {
                        toggle_play_pause(Arc::clone(&sink));
                    }
                    KeyCode::Char(',') => {
                        set_play_speed(Arc::clone(&sink), 0.75);
                    }
                    KeyCode::Char('.') => {
                        set_play_speed(Arc::clone(&sink), 1.0);
                    }
                    KeyCode::Char('/') => {
                        set_play_speed(Arc::clone(&sink), 1.5);
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
