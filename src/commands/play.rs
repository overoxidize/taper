use rodio::{
    source::{self, SineWave, Source},
    Decoder, OutputStream, Sink,
};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

pub fn play_audio(source_audio: PathBuf) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // OutputStream is a wrapper over the `cpal` `Stream` struct, which represents
    // an open flow of audio data.
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);
    sink.append(source);

    sink.sleep_until_end();
}
