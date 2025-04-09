use crate::handlers::sink_handler::SinkHandler;

use rodio::OutputStream;
use std::{error::Error, path::PathBuf, sync::Arc, thread};

/// Encapsulates audio-related state and controls.
pub struct InputHandler {
    pub _stream: OutputStream,
    pub audio_player: Arc<SinkHandler>,
    pub vol: i16,
    pub paused: bool,
}
impl InputHandler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let audio_player = Arc::new(SinkHandler::new(stream_handle));
        Ok(Self {
            _stream: stream,
            audio_player,
            vol: 100,
            paused: false,
        })
    }

    /// Starts playing the file on a new thread using the AudioPlayer.
    pub fn play(&mut self, path: &PathBuf) {
        let path_clone = path.clone();
        let current_vol = self.vol;
        let sink_handler = Arc::clone(&self.audio_player);
        thread::spawn(move || {
            sink_handler.play_file(path_clone, current_vol);
        });
        self.paused = false;
    }

    /// Append audio to the sink.
    pub fn append(&mut self, path: &PathBuf) {
        let path_clone = path.clone();
        let current_vol = self.vol;
        let sink_handler = Arc::clone(&self.audio_player);
        thread::spawn(move || {
            sink_handler.append_to_sink(path_clone, current_vol);
        });
    }

    /// Removes all currently loaded Sources from the Sink, and pauses it.
    pub fn clear_sink(&self) {
        self.audio_player.clear();
    }

    /// Toggles between play and pause.
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.audio_player.toggle_play_pause();
    }

    /// Adjusts the volume by a given delta.
    pub fn adjust_volume(&mut self, delta: i16) {
        let new_vol = self.vol + delta;
        if new_vol >= 0 && new_vol <= 100 {
            self.vol = new_vol;
            self.audio_player.set_volume(self.vol);
        }
    }

    /// Returns the sink's position in seconds.
    pub fn sink_pos(&self) -> u64 {
        self.audio_player.sink_pos()
    }

    /// Returns the sink's position in milliseconds.
    pub fn sink_pos_millis(&self) -> u128 {
        self.audio_player.sink_pos_millis()
    }

    /// Returns the sink's length.
    pub fn get_len(&self) -> usize {
        self.audio_player.get_len()
    }

    /// Returns true if the sink is empty, otherwise false.
    pub fn is_empty(&self) -> bool {
        self.audio_player.is_empty()
    }
}
