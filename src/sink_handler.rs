use rodio::{Decoder, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, sync::Mutex, time::Duration};

/// Encapsulates an audio sink and an output stream handle
pub struct SinkHandler {
    stream_handle: OutputStreamHandle,
    sink: Mutex<Option<Sink>>,
}

impl SinkHandler {
    /// Creates a new SinkHandler with the given stream handle
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self {
            stream_handle,
            sink: Mutex::new(None),
        }
    }

    /// Plays a FLAC file and sets its initial volume
    pub fn play_file(&self, path: PathBuf, vol: i16) {
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).expect("Failed to decode audio");

        let new_sink = Sink::try_new(&self.stream_handle).expect("Failed to create sink");
        new_sink.append(source);

        // Store the sink in the player's state
        *self.sink.lock().unwrap() = Some(new_sink);

        self.set_volume(vol);
    }

    /// Toggles play and pause
    pub fn toggle_play_pause(&self) {
        let sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            match sink.is_paused() {
                true => {
                    sink.play();
                }
                false => {
                    sink.pause();
                }
            }
        }
    }

    /// Sets the playback speed
    pub fn set_play_speed(&self, mag: i16) {
        let sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            sink.set_speed((mag as f32) / 100.0);
        }
    }

    /// Sets the playback volume
    pub fn set_volume(&self, mag: i16) {
        let sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            sink.set_volume((mag as f32) / 100.0);
        }
    }

    /// Gets the sink's position in seconds
    pub fn sink_pos(&self) -> u64 {
        let sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            sink.get_pos().as_secs()
        } else {
            Duration::new(0, 0).as_secs()
        }
    }

    /*
    pub fn get_volume(&self) -> u8 {
        let sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            (sink.volume() * 100.0).round() as u8
        } else {
            100
        }
    }

    pub fn get_len(&self) -> usize {
        let sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            sink.len()
        } else {
            0
        }
    }
    */
}
