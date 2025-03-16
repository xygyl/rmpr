use crate::audio::{get_vol, play_file, set_play_speed, set_vol, toggle_play_pause, SharedSink};
// use audiotags::Tag;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState},
    Terminal,
};
use rodio::OutputStream;
use std::{
    collections::HashMap, env, fs, io, path::PathBuf, str::FromStr, sync::Arc, thread,
    time::Duration,
};

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

    let mut play_speed: f32 = 1.0;
    let mut vol: f32 = 1.0;
    let mut paused: bool = false;
    let mut muted: bool = false;

    let mut playing_file: Option<String> = None;
    let mut sel_map: HashMap<PathBuf, usize> = HashMap::new();
    sel_map.insert(current_dir.clone(), 0);

    // Shared output stream and sink for audio control
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink: SharedSink = Arc::new(std::sync::Mutex::new(None));

    loop {
        let mut directories: Vec<PathBuf> = Vec::new();
        let mut files: Vec<PathBuf> = Vec::new();

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
                } else {
                    files.push(path);
                }
            }
        }

        directories.sort();
        files.sort();

        let entries: Vec<PathBuf> = directories.into_iter().chain(files.into_iter()).collect();

        let items: Vec<ListItem> = entries
            .iter()
            .map(|entry| {
                let file_name = entry
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| String::from("Unknown"));

                let is_dir = entry.is_dir();
                let display = if is_dir {
                    format!("{}", file_name)
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

            // bottom right
            let bottom = Line::from(vec![
                Span::styled(
                    format!("Paused: {:>5}", paused),
                    Style::default().fg(Color::from_str("#417BFF").unwrap()),
                ),
                Span::styled(
                    " | ",
                    Style::default().fg(Color::from_str("#00ffaa").unwrap()),
                ),
                Span::styled(
                    format!("Muted: {:>5}", muted),
                    Style::default().fg(Color::from_str("#AE5DFF").unwrap()),
                ),
                Span::styled(
                    " | ",
                    Style::default().fg(Color::from_str("#00ffaa").unwrap()),
                ),
                // Span::styled(
                //     format!("Volume: {:<2.2}%", vol),
                //     Style::default().fg(Color::from_str("#FF5DC8").unwrap()),
                // ),
                Span::styled(
                    format!("Volume: {:<2.2}%", get_vol(Arc::clone(&sink))),
                    Style::default().fg(Color::from_str("#FF5DC8").unwrap()),
                ),
            ]);

            let block = Block::bordered()
                .border_style(Style::default().fg(Color::from_str("#1F153E").unwrap()))
                .border_set(border::THICK)
                // top left
                .title_top(
                    Line::from(Span::styled(
                        format!("{}", current_dir.display()),
                        Style::default().fg(Color::from_str("#00FFAA").unwrap()),
                    ))
                    .left_aligned(),
                )
                // top right
                .title_top(
                    Line::from(Span::styled(
                        format!("Playback speed: x{:<4}", play_speed),
                        Style::default().fg(Color::from_str("#FF5D85").unwrap()),
                    ))
                    .right_aligned(),
                )
                // bottom left
                .title_bottom(Line::from(match &playing_file {
                    Some(file) => Span::styled(
                        format!("Playing: {}", file),
                        Style::default().fg(Color::from_str("#F1FF5D").unwrap()),
                    ),
                    None => String::new().into(),
                }))
                .title_bottom(bottom.right_aligned());

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default().fg(Color::from_str("#00EAFF").unwrap()));

            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,

                    // play selected file
                    KeyCode::Enter => {
                        if let Some(path) = entries.get(selected) {
                            let stream_handle_clone = stream_handle.clone();
                            let sink_clone = Arc::clone(&sink);
                            let path_clone = path.clone();
                            thread::spawn(move || {
                                play_file(path_clone, stream_handle_clone, sink_clone);
                            });
                            playing_file = path
                                .file_name()
                                .map(|name| name.to_string_lossy().to_string());
                        }
                        play_speed = 1.0; // Reset speed when new song is played
                        muted = false;
                        paused = false;
                        // TODO: fix volume not persisting
                        set_vol(Arc::clone(&sink), vol);
                    }

                    // file system movement
                    KeyCode::Left | KeyCode::Char('h') => {
                        if let Some(parent) = current_dir.parent() {
                            selected = *sel_map.get(parent).unwrap_or(&0);
                            current_dir = parent.to_path_buf();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected < entries.len() - 1 {
                            selected += 1;
                        } else {
                            selected = 0;
                        }
                        sel_map.insert(current_dir.clone(), selected);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if selected == 0 {
                            selected = entries.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        sel_map.insert(current_dir.clone(), selected);
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if let Some(path) = entries.get(selected) {
                            if path.is_dir() {
                                selected = *sel_map.get(path).unwrap_or(&0);
                                current_dir = path.clone();
                            }
                        }
                    }

                    // playback speed
                    KeyCode::Char(',') | KeyCode::Char('<') => {
                        if play_speed == 0.25 {
                            continue;
                        } else {
                            play_speed -= 0.25;
                        }
                        set_play_speed(Arc::clone(&sink), play_speed);
                    }
                    KeyCode::Char('.') | KeyCode::Char('>') => {
                        if play_speed == 2.0 {
                            continue;
                        } else {
                            play_speed += 0.25;
                        }
                        set_play_speed(Arc::clone(&sink), play_speed);
                    }
                    KeyCode::Char('/') => {
                        play_speed = 1.0;
                        set_play_speed(Arc::clone(&sink), play_speed);
                    }

                    // mute
                    KeyCode::Char('m') => match muted {
                        false => {
                            set_vol(Arc::clone(&sink), 0.0);
                            muted = true;
                        }
                        true => {
                            set_vol(Arc::clone(&sink), vol);
                            muted = false;
                        }
                    },

                    // play/pause
                    KeyCode::Char('p') => {
                        match paused {
                            true => {
                                paused = false;
                            }
                            false => {
                                paused = true;
                            }
                        }
                        toggle_play_pause(Arc::clone(&sink));
                    }

                    // volume
                    KeyCode::Char('-') | KeyCode::Char('_') => {
                        if vol <= 0.0 {
                            vol = 0.0;
                            continue;
                        } else {
                            vol -= 0.05;
                        }
                        set_vol(Arc::clone(&sink), vol);
                    }
                    KeyCode::Char('=') | KeyCode::Char('+') => {
                        if vol >= 1.0 {
                            vol = 1.0;
                            continue;
                        } else {
                            vol += 0.05;
                        }
                        set_vol(Arc::clone(&sink), vol);
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
