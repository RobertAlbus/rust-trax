use std::time::Duration;

use rodio::{cpal::Stream, OutputStream, Source};

pub struct StreamAdapter<'a> {
    sample_rate: u32,
    sources: Vec<&'a mut Box<dyn Source<Item = f32>>>,
}

unsafe impl Send for StreamAdapter<'_> {}

impl<'a> StreamAdapter<'a> {
    pub fn new(sample_rate: u32) -> StreamAdapter<'static> {
        StreamAdapter {
            sample_rate,
            sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, mut source: &'a mut Box<dyn Source<Item = f32>>) {
        self.sources.push(source);
    }

    pub fn get_sample(&self) -> f32 {
        self.sources
            .iter()
            .map(|source| source.next().unwrap())
            .sum::<f32>()
            / self.sources.len() as f32
    }
}

impl Iterator for StreamAdapter<'_> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.get_sample())
    }
}

impl Source for StreamAdapter<'_> {
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
