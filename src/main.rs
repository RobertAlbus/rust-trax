// https://youtu.be/v0Qp7eWVyes?t=723
#![allow(dead_code)]

use std::time::Duration;

use rodio::{OutputStream, Source};


struct WavetableOscillator {
    sample_rate: u32,
    wavetable: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wavetable: Vec<f32>) -> WavetableOscillator {
        WavetableOscillator {
            sample_rate,
            wavetable,
            index: 0.,
            index_increment: 0.
        }
    }

    fn set_frequency(&mut self, freq: f32) {
        self.index_increment = freq * self.wavetable.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wavetable.len() as f32;
        sample
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wavetable.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wavetable[truncated_index] + 
            next_index_weight * self.wavetable[next_index];
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.get_sample())
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    let wavetable_size = 64;
    let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);

    for n in 0..wavetable_size {
        let sample = (2. * std::f32::consts::PI * n as f32 / wavetable_size as f32).sin();
        wavetable.push(sample);
    }

    let mut oscillator = WavetableOscillator::new(48_000, wavetable); 
    oscillator.set_frequency(440.);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_secs(5));
}
