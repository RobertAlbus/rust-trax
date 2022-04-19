// https://youtu.be/v0Qp7eWVyes?t=723
#![allow(dead_code)]
#![allow(unused_imports)]

use std::time::Duration;
use rodio::{OutputStream, Source};
mod wavetable;
use wavetable::WavetableOscillator;

mod wave_shape;
use wave_shape::{bit_crush, square, pure, something, shape, gain};

mod generate;
use generate::sin;

mod visualize;

fn main() {
    let sample_rate = 48_000;
    let playback_duration = Duration::from_secs(10);
    let freq = 40.;
    let wavetable_size = 2048;
    
    let mut wavetable = generate::sin(wavetable_size);
    shape(&mut wavetable, vec![
        Box::new(something), 
        Box::new(bit_crush(4.)), 
        Box::new(gain(0.5)), 
    ]);

    visualize::png(&wavetable, "wave.png");

    let mut oscillator = WavetableOscillator::new(sample_rate, wavetable); 
    oscillator.set_frequency(freq);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(playback_duration);
}

