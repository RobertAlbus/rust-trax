// https://youtu.be/v0Qp7eWVyes?t=723
#![allow(dead_code)]
#![allow(unused_imports)]

use std::time::Duration;
use rodio::{OutputStream, Source};
mod wavetable;
use wavetable::WavetableOscillator;

mod wave_shape;
use wave_shape::{bit_crush, square};


fn main() {
    let wavetable_size = 64;
    let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);
    let shape_fn = bit_crush(1.2);

    for n in 0..wavetable_size {
        let sample = (2. * std::f32::consts::PI * n as f32 / wavetable_size as f32).sin();
        wavetable.push(shape_fn(sample));
    }

    let mut oscillator = WavetableOscillator::new(48_000, wavetable); 
    oscillator.set_frequency(55.);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_secs(5));
}
