// AI generated stuff

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::Cursor;

const BEEP_WAV: &[u8] = include_bytes!("..\\..\\assets\\beep.wav");

pub struct SoundPlayer {
    _stream: OutputStream, // must be held to keep audio alive
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>,
}

impl SoundPlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            _stream,
            stream_handle,
            sink: None,
        }
    }

    pub fn ensure_playing(&mut self) {
        if self.sink.is_some() {
            return; // already playing
        }

        let source = Decoder::new(Cursor::new(BEEP_WAV)).unwrap().repeat_infinite();
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);
        sink.play();
        self.sink = Some(sink);
    }

    pub fn ensure_stopped(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
    }
}
