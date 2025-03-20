use crate::sink_handling::{
    get_len, play_file, set_play_speed, set_vol, toggle_play_pause, SharedSink,
};
use rodio::{OutputStream, OutputStreamHandle};
use std::{path::PathBuf, sync::Arc, thread};

/// Encapsulates audio-related state and controls
pub struct AudioPlaying {
    pub _stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
    pub sink: SharedSink,
    pub playing_file: Option<String>,
    pub play_speed: i16,
    pub vol: i16,
    pub paused: bool,
    pub muted: bool,
}

impl AudioPlaying {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Arc::new(std::sync::Mutex::new(None));
        Ok(Self {
            _stream: stream,
            stream_handle,
            sink,
            playing_file: None,
            play_speed: 100,
            vol: 100,
            paused: false,
            muted: false,
        })
    }

    pub fn play(&mut self, path: &PathBuf) {
        let path_clone = path.clone();
        let sink_clone = Arc::clone(&self.sink);
        let stream_handle_clone = self.stream_handle.clone();
        let current_vol = self.vol;
        thread::spawn(move || {
            play_file(path_clone, stream_handle_clone, sink_clone, current_vol);
        });
        self.playing_file = path.file_name().map(|n| n.to_string_lossy().to_string());
        self.play_speed = 100;
        self.muted = false;
        self.paused = false;
    }

    pub fn adjust_speed(&mut self, delta: i16) {
        let new_speed = self.play_speed + delta;
        if new_speed >= 25 && new_speed <= 200 {
            self.play_speed = new_speed;
            set_play_speed(Arc::clone(&self.sink), self.play_speed);
        }
    }

    pub fn reset_speed(&mut self) {
        self.play_speed = 100;
        set_play_speed(Arc::clone(&self.sink), self.play_speed);
    }

    pub fn toggle_mute(&mut self) {
        if !self.muted {
            set_vol(Arc::clone(&self.sink), 0);
            self.muted = true;
        } else {
            set_vol(Arc::clone(&self.sink), self.vol);
            self.muted = false;
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        toggle_play_pause(Arc::clone(&self.sink));
    }

    pub fn adjust_volume(&mut self, delta: i16) {
        let new_vol = self.vol + delta;
        if new_vol >= 0 && new_vol <= 100 && get_len(Arc::clone(&self.sink)) > 0 {
            self.vol = new_vol;
            set_vol(Arc::clone(&self.sink), self.vol);
        }
    }
}
