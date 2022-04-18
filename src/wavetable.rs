use std::time::Duration;
use rodio::Source;

pub struct WavetableOscillator {
    sample_rate: u32,
    wavetable: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wavetable: Vec<f32>) -> WavetableOscillator {
        WavetableOscillator {
            sample_rate,
            wavetable,
            index: 0.,
            index_increment: 0.
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.index_increment = freq * self.wavetable.len() as f32 / self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> f32 {
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
