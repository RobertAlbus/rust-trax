// https://youtu.be/v0Qp7eWVyes?t=723
#![allow(dead_code)]
#![allow(unused_imports)]

use console::Term;
use rodio::{OutputStream, Source};
use std::time::Duration;

mod wavetable;
use wavetable::WavetableOscillator;

mod wave_shape;
use wave_shape::{bit_crush, clip, gain, pure, shape, something, square};

mod generate;
use generate::sin;

mod visualize;

pub fn main() {
    let sample_rate = 48_000;
    let freq = 40.;
    let wavetable_size = 2048;

    let mut wavetable = generate::sin(wavetable_size);

    for _ in 0..1 {
        shape(&mut wavetable, vec![
            // gain(1.1), 
            clip(0.55),
            // bit_crush(22.),
            ]
        );
    }

    // shape(&mut wavetable, vec![gain(0.1)]);

    visualize::png(&wavetable, "wave.png");

    let mut oscillator = WavetableOscillator::new(sample_rate, wavetable);
    oscillator.set_frequency(freq);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    let stdout = Term::buffered_stdout();
    'program_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                _ => break 'program_loop,
            }
        }
    }
}
