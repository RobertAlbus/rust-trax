pub fn sin(wavetable_size: usize) -> Vec<f32> {

  let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);
  for n in 0..wavetable_size {
    let sample = (2. * std::f32::consts::PI * n as f32 / wavetable_size as f32).sin();
    wavetable.push(sample);
  }
  
  wavetable
}

pub fn saw(wavetable_size: usize) -> Vec<f32> {

  let increment = 2. / wavetable_size as f32;
  let mut accumulator = -1.0;

  let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);
  for _ in 0..wavetable_size {
    wavetable.push(accumulator);
    accumulator += increment;
  }
  
  wavetable
}
