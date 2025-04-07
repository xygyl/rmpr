use rodio::{Decoder, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, sync::Mutex, time::Duration};

/// Encapsulates an audio sink and an output stream handle
pub struct SinkHandler {
    stream_handle: OutputStreamHandle,
    sink: Mutex<Option<Sink>>,
}

impl SinkHandler {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self {
            stream_handle,
            sink: Mutex::new(None),
        }
    }

    /// Plays the given file and sets its volume
    pub fn play_file(&self, path: PathBuf, vol: i16) {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).unwrap();

        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);

        // Store the sink in the player's state
        *self.sink.lock().unwrap() = Some(sink);

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
        match &*sink_guard {
            Some(sink) => sink.get_pos().as_secs(),
            None => Duration::new(0, 0).as_secs(),
        }
    }

    /// Gets the sink's position in milliseconds
    pub fn sink_pos_millis(&self) -> u128 {
        let sink_guard = self.sink.lock().unwrap();
        match &*sink_guard {
            Some(sink) => sink.get_pos().as_millis(),
            None => Duration::new(0, 0).as_millis(),
        }
    }

    /// Appends source to sink
    pub fn append_to_sink(&self, path: PathBuf, vol: i16) {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).unwrap();

        {
            let sink_guard = self.sink.lock().unwrap();
            if let Some(ref sink) = *sink_guard {
                if sink.len() >= 1 {
                    sink.append(source);
                }
            }
        } // Lock is dropped here. Prevents deadlocks.

        self.set_volume(vol);
    }

    /// Removes all currently loaded Sources from the Sink, and pauses it
    pub fn clear(&self) {
        let sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            sink.clear();
        }
    }

    /// Returns how many elements are in the sink
    pub fn get_len(&self) -> usize {
        let sink_guard = self.sink.lock().unwrap();
        match &*sink_guard {
            Some(sink) => sink.len(),
            None => 0,
        }
    }

    /// Returns true if the sink is empty, otherwise false
    pub fn is_empty(&self) -> bool {
        let sink_guard = self.sink.lock().unwrap();
        match &*sink_guard {
            Some(sink) => sink.empty(),
            None => true,
        }
    }
}
