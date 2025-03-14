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
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::{
    env, fs, io,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

// Shared OutputStreamHandle to keep audio alive
type SharedSink = Arc<Mutex<Option<Sink>>>;

// Plays a FLAC file using rodio on a separate thread.
fn play_flac_file(path: PathBuf, stream_handle: OutputStreamHandle, sink: SharedSink) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let source = Decoder::new(reader).unwrap();
    let new_sink = Sink::try_new(&stream_handle).unwrap();

    new_sink.append(source);

    // Store the sink in shared state
    *sink.lock().unwrap() = Some(new_sink);
}

fn toggle_play_pause(sink: SharedSink) {
    let sink_guard = sink.lock().unwrap();
    if let Some(sink) = &*sink_guard {
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_dir = env::current_dir()?;
    let mut selected: usize = 0;
    let mut list_state = ListState::default();
    list_state.select(Some(selected));

    // Shared output stream and sink for audio control
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Arc::new(Mutex::new(None));

    loop {
        let mut entries: Vec<PathBuf> = fs::read_dir(&current_dir)?
            .filter_map(|entry| {
                entry.ok().map(|e| e.path()).filter(|path| {
                    if let Some(file_name) = path.file_name() {
                        !file_name.to_string_lossy().starts_with('.')
                    } else {
                        false
                    }
                })
            })
            .collect();
        entries.sort();

        let items: Vec<ListItem> = entries
            .iter()
            .map(|entry| {
                let file_name = entry
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| String::from("Unknown"));
                let display = if entry.is_dir() {
                    format!("{}/", file_name)
                } else {
                    file_name
                };
                ListItem::new(display)
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
                .title(format!("Directory: {}", current_dir.display()));

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default().fg(Color::Blue))
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
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
                    KeyCode::Enter => {
                        if let Some(path) = entries.get(selected) {
                            if let Some(ext) = path.extension() {
                                if ext.to_string_lossy().to_lowercase() == "flac" {
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
                                // Enter directory.
                                current_dir = path.clone();
                                selected = 0;
                            }
                        }
                    }
                    KeyCode::Left => {
                        if let Some(parent) = current_dir.parent() {
                            current_dir = parent.to_path_buf();
                            selected = 0;
                        }
                    }
                    KeyCode::Char(' ') => {
                        toggle_play_pause(Arc::clone(&sink));
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
