// https://youtu.be/v0Qp7eWVyes?t=723
#![allow(dead_code)]
#![allow(unused_imports)]

use std::time::Duration;
use rodio::{OutputStream, Source};
mod wavetable;
use wavetable::WavetableOscillator;

mod wave_shape;
use wave_shape::{bit_crush, square, pure, something};

mod generate;
use generate::generate_sin;


fn main() {
    let sample_rate = 48_000;
    let playback_duration = Duration::from_secs(10);
    let freq = 40.;
    let shape_fn = square;
    let shape_fn: Option<Box<dyn Fn(f32)->f32>> = Some(Box::new(shape_fn));
    let wavetable_size = 1048;

    let wavetable = generate::generate_sin(wavetable_size, shape_fn);

    let mut oscillator = WavetableOscillator::new(sample_rate, wavetable); 
    oscillator.set_frequency(freq);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(playback_duration);
}

