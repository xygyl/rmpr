use crate::{data::metadata::file_data::FileData, tui::tui::App};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

/// Handles events
impl App {
    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    /// Handles key events
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        let speed_delta = self.config.controls.speed_delta;
        let audio_delta = self.config.controls.audio_delta;

        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

            KeyCode::Enter => {
                if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
                    if !path.is_dir() {
                        match self.audio.sink_len() {
                            0 => {
                                self.audio.play(path);

                                self.meta_manager
                                    .update_current(FileData::new(), path, true);
                                self.data = self.meta_manager.current.clone();

                                self.path_queue.push(path.clone());
                                self.name.push(
                                    path.clone()
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string(),
                                );
                            }
                            _ => {
                                self.path_queue.insert(0, path.clone());
                                self.name.insert(
                                    0,
                                    path.clone()
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string(),
                                );
                                self.audio.clear_sink();
                                self.audio.play(&self.path_queue[0]);
                                for element in self.path_queue.iter().skip(1) {
                                    self.audio.append(element);
                                }
                                self.meta_manager
                                    .update_current(FileData::new(), path, false);
                                self.data = self.meta_manager.current.clone();
                            }
                        }
                    }
                }
            }

            KeyCode::Char('a') => {
                if let Some(path) = self.file_browser.entries.get(self.file_browser.selected) {
                    if !path.is_dir() {
                        match self.audio.sink_len() {
                            0 => {
                                self.audio.play(path);

                                self.meta_manager
                                    .update_current(FileData::new(), path, true);
                                self.data = self.meta_manager.current.clone();
                            }
                            _ => {
                                self.audio.append(path);
                                self.meta_manager.queue_metadata(FileData::new(), path);
                            }
                        }
                    }
                    self.path_queue.push(path.clone());
                    self.name.push(
                        path.clone()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );
                }
            }

            KeyCode::Char('s') => {
                if self.audio.sink_len() > 0 {
                    self.path_queue.remove(0);
                    self.name.remove(0);
                    self.audio.clear_sink();

                    if let Some(next_path) = self.path_queue.get(0) {
                        self.audio.play(next_path);
                        for element in self.path_queue.iter().skip(1) {
                            self.audio.append(element);
                        }

                        self.data = self.meta_manager.pop_next().unwrap_or(FileData {
                            raw_file: None,
                            album: None,
                            artist: None,
                            title: None,
                            year: None,
                            duration_display: None,
                            duration_as_secs: None,
                            track_number: None,
                        });
                    } else {
                        self.data = FileData {
                            raw_file: None,
                            album: None,
                            artist: None,
                            title: None,
                            year: None,
                            duration_display: None,
                            duration_as_secs: None,
                            track_number: None,
                        };
                    }
                }
            }

            KeyCode::Up | KeyCode::Char('k') => self.file_browser.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.file_browser.navigate_down(),
            KeyCode::Left | KeyCode::Char('h') => self.file_browser.navigate_back(),
            KeyCode::Right | KeyCode::Char('l') => self.file_browser.navigate_into(),

            KeyCode::PageUp => self.file_browser.goto_top(),
            KeyCode::PageDown => self.file_browser.goto_bottom(),

            KeyCode::Char('g') => {
                self.file_browser.current_dir = self.config.directories.music_directory.clone()
            }

            KeyCode::Char('c') => {
                self.audio.clear_sink();
                self.path_queue.clear();
                self.name.clear();
            }

            KeyCode::Char('.') | KeyCode::Char('>') => self.audio.adjust_speed(speed_delta),
            KeyCode::Char(',') | KeyCode::Char('<') => self.audio.adjust_speed(speed_delta * -1),
            KeyCode::Char('/') => self.audio.reset_speed(),

            KeyCode::Char('=') | KeyCode::Char('+') => self.audio.adjust_volume(audio_delta),
            KeyCode::Char('-') | KeyCode::Char('_') => self.audio.adjust_volume(audio_delta * -1),
            KeyCode::Char('m') => self.audio.toggle_mute(),

            KeyCode::Char('p') => self.audio.toggle_pause(),

            _ => {}
        }
    }
}
