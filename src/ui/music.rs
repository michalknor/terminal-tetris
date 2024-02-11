use rodio::source::SineWave;
use rodio::OutputStream;
use std::thread;
use std::time::Duration;

pub async fn play_theme() -> Result<(), std::io::Error> {
    tokio::spawn(async {
        let frequencies = [
            659.25511, 493.8833, 523.25113, 587.32954, 523.25113, 493.8833, 440.0, 440.0, 
            523.25113, 659.25511, 587.32954, 523.25113, 493.8833, 523.25113, 587.32954, 
            659.25511, 523.25113, 440.0, 440.0, 440.0, 493.8833, 523.25113, 587.32954, 
            698.45646, 880.0, 783.99087, 698.45646, 659.25511, 523.25113, 659.25511, 
            587.32954, 523.25113, 493.8833, 493.8833, 523.25113, 587.32954, 659.25511, 
            523.25113, 440.0, 440.0
        ];
        let durations = [
            406.250, 203.125, 203.125, 406.250, 203.125, 203.125, 406.250, 203.125, 
            203.125, 406.250, 203.125, 203.125, 609.375, 203.125, 406.250, 406.250, 
            406.250, 406.250, 203.125, 203.125, 203.125, 203.125, 609.375, 203.125, 
            406.250, 203.125, 203.125, 609.375, 203.125, 406.250, 203.125, 203.125, 
            406.250, 203.125, 203.125, 406.250, 406.250, 406.250, 406.250, 406.250
        ];

        loop {
            for i in 0..durations.len() {
                play_sound(frequencies[i], durations[i] as u64);
            }
        }
    });

    Ok(())
}

fn play_sound(frequency: f64, duration_ms: u64) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(0.1);

    let source = SineWave::new(frequency as u32);

    sink.append(source);

    thread::sleep(Duration::from_millis(duration_ms));
}