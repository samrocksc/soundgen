use light_morse::*;
use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;
use rodio::{
    default_output_device, play_raw,
    source::{SineWave, Source},
};

pub trait Display {
    fn display(&self);
}

pub fn sound(cool: char) {
    let device = default_output_device().unwrap();
    let source = SineWave::new(440);
    let short = source.take_duration(Duration::from_millis(200));
    let source2 = SineWave::new(440);
    let long = source2.take_duration(Duration::from_millis(500));
    match cool {
        '−' => play_raw(&device, long.convert_samples()),
        '·' => play_raw(&device, short.convert_samples()),
        _ => println!("nope")
    }
}

impl Display for Morse {
    fn display(&self) {
        for item in self.chars() {
            sound(item);
            print!("{}", item);
            stdout().flush().unwrap();

            thread::sleep(Duration::from_millis(800));
        }
        println!();
    }
}

