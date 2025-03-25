use rodio::{Decoder, OutputStreamHandle, Sink};
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Shared sink type to control audio playback.
pub type SharedSink = Arc<Mutex<Option<Sink>>>;

/// Plays a FLAC file on a separate thread.
pub fn play_file(path: PathBuf, stream_handle: OutputStreamHandle, sink: SharedSink, vol: i16) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let source = Decoder::new(reader).unwrap();

    let new_sink = Sink::try_new(&stream_handle).unwrap();
    new_sink.append(source);

    // Store the sink in shared state
    *sink.lock().unwrap() = Some(new_sink);

    set_vol(Arc::clone(&sink), vol);
}

// TODO: add a function to add songs to sink queue and a way to display that at the bottom left of tui

pub fn toggle_play_pause(sink: SharedSink) {
    let sink_guard = sink.lock().unwrap();
    if let Some(sink) = &*sink_guard {
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }
}

pub fn set_play_speed(sink: SharedSink, mag: i16) {
    let sink_guard = sink.lock().unwrap();
    if let Some(sink) = &*sink_guard {
        sink.set_speed((mag as f32) / 100.0);
    }
}

pub fn set_vol(sink: SharedSink, mag: i16) {
    let sink_guard = sink.lock().unwrap();
    if let Some(sink) = &*sink_guard {
        sink.set_volume((mag as f32) / 100.0);
    }
}
/* pub fn get_vol(sink: SharedSink) -> u8 {
    let sink_guard = sink.lock().unwrap();
    if let Some(sink) = &*sink_guard {
        (sink.volume() * 100.0).round() as u8
    } else {
        100
    }
} */

/* pub fn get_len(sink: SharedSink) -> usize {
    let sink_guard = sink.lock().unwrap();
    if let Some(sink) = &*sink_guard {
        sink.len()
    } else {
        0
    }
} */
