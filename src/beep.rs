use std::time::Duration;
use std::io::BufReader;
use std::thread;
use rodio;

pub fn play_beep(volume: f32) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file = std::fs::File::open("assets/beep.wav").unwrap();
    let beep = stream_handle.play_once(BufReader::new(file)).unwrap();
    beep.set_volume(volume);
    beep.play();
    thread::sleep(Duration::from_millis(150));
}