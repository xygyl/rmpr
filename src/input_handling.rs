use crate::sink_handling::AudioPlayer;
use rodio::OutputStream;
use std::{path::PathBuf, sync::Arc, thread};

/// Encapsulates audio-related state and controls.
pub struct HandleInput {
    pub _stream: OutputStream,
    pub audio_player: Arc<AudioPlayer>,
    pub play_speed: i16,
    pub vol: i16,
    pub paused: bool,
    pub muted: bool,
}

impl HandleInput {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let audio_player = Arc::new(AudioPlayer::new(stream_handle));
        Ok(Self {
            _stream: stream,
            audio_player,
            play_speed: 100,
            vol: 100,
            paused: false,
            muted: false,
        })
    }

    /// Starts playing the file on a new thread using the AudioPlayer
    pub fn play(&mut self, path: &PathBuf) {
        let path_clone = path.clone();
        let current_vol = self.vol;
        let audio_player = Arc::clone(&self.audio_player);
        thread::spawn(move || {
            audio_player.play_file(path_clone, current_vol);
        });
        self.play_speed = 100;
        self.muted = false;
        self.paused = false;
    }

    /// Adjusts the playback speed by a given delta
    pub fn adjust_speed(&mut self, delta: i16) {
        let new_speed = self.play_speed + delta;
        if new_speed >= 25 && new_speed <= 200 {
            self.play_speed = new_speed;
            self.audio_player.set_play_speed(self.play_speed);
        }
    }

    /// Resets the playback speed to normal
    pub fn reset_speed(&mut self) {
        self.play_speed = 100;
        self.audio_player.set_play_speed(self.play_speed);
    }

    /// Toggles mute on and off
    pub fn toggle_mute(&mut self) {
        match self.muted {
            true => {
                self.audio_player.set_volume(self.vol);
                self.muted = false;
            }
            false => {
                self.audio_player.set_volume(0);
                self.muted = true;
            }
        }
    }

    /// Toggles between play and pause
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.audio_player.toggle_play_pause();
    }

    /// Adjusts the volume by a given delta
    pub fn adjust_volume(&mut self, delta: i16) {
        let new_vol = self.vol + delta;
        if new_vol >= 0 && new_vol <= 100 {
            self.vol = new_vol;
            self.audio_player.set_volume(self.vol);
        }
    }

    pub fn sink_pos(&self) -> u64 {
        self.audio_player.sink_pos().as_secs()
    }
}
