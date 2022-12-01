use std::time::Duration;
use std::io::BufReader;
use std::thread;
use rodio;

pub fn play_beep(volume: f32) -> bool {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::fs::File::open("assets/beep.wav");
    // let file = std::fs::File::open("/Applications/oxidized-timer.app/assets/beep.wav");

    match file {
        Ok(file) => {
            // this part is bad, but it is only for me
            let beep = stream_handle.play_once(BufReader::new(file)).unwrap();
            beep.set_volume(volume);
            beep.play();
            thread::sleep(Duration::from_millis(150));
            true
        },
        Err(_) => {
            false
        },
    }
}