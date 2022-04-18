// https://youtu.be/v0Qp7eWVyes?t=723
#![allow(dead_code)]

use std::time::Duration;

use rodio::{OutputStream, Source};
mod wavetable;
use wavetable::WavetableOscillator;


fn main() {
    let wavetable_size = 64;
    let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);
    let shape_fn = bit_crush(1.2);

    for n in 0..wavetable_size {
        let sample = (2. * std::f32::consts::PI * n as f32 / wavetable_size as f32).sin();
        wavetable.push(shape_fn(sample));
    }

    let mut oscillator = WavetableOscillator::new(48_000, wavetable); 
    oscillator.set_frequency(440.);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_secs(5));
}

fn square(sample: f32) -> f32 {
    pwm(0.)(sample)
}

fn rect(sample: f32) -> f32 {
    pwm(0.8)(sample)
}

fn pwm(threshold: f32) -> Box<dyn Fn(f32)->f32> {
    Box::new(move |sample| {
        let width = f32::from(threshold);
        if sample > width {
            1_f32
        } else {
            -1_f32
        }
    })

}

fn bit_crush(bits: f32) -> Box<dyn Fn(f32)->f32> {
    Box::new(move |sample: f32| {
        let mut sample = (sample + 2.) / 2.;
        sample = (sample * bits).floor() / (bits -1.);
        sample = sample * 2. - 2.;
        
        sample
    })
}
