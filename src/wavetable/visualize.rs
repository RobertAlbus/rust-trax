use audio_visualizer::{waveform, Channels};


pub fn console(wavetable: &Vec<f32>) {
  wavetable.iter().for_each(|i| {
    println!("{}",i);
  });
}

pub fn png(wavetable: &Vec<f32>, filename: &str) {
  let mut normalized = normalize(wavetable);
  normalized = multicycle(normalized, 4);
  waveform::png_file::waveform_static_png_visualize(
    &normalized[..], Channels::Mono, "./img", filename);
}

pub fn normalize(wavetable: &Vec<f32>) -> Vec<i16> {
  wavetable.iter().map(|s| {
    (s * 32767.) as i16
  }).collect()
}

pub fn multicycle(wavetable: Vec<i16>, cycles: usize) -> Vec<i16> {
  let mut multicycle_wavetable: Vec<i16> = Vec::with_capacity(wavetable.capacity());
  for _ in 0..cycles {
    let mut dupe: Vec<i16> = wavetable.clone();
    multicycle_wavetable.append(&mut dupe);
  }

  multicycle_wavetable
}


