use crate::wave_shape::pure;

pub fn sin(
  wavetable_size: usize,
  shape_fn: Option<Box<dyn Fn(f32)->f32>>
) -> Vec<f32> {

  let shape_fn = get_shape_fn(shape_fn);

  let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);
  for n in 0..wavetable_size {
    let sample = (2. * std::f32::consts::PI * n as f32 / wavetable_size as f32).sin();
    wavetable.push(shape_fn(sample));
  }
  
  wavetable
}

pub fn saw(
  wavetable_size: usize,
  shape_fn: Option<Box<dyn Fn(f32)->f32>>
) -> Vec<f32> {

  let shape_fn = get_shape_fn(shape_fn);
  let increment = 2. / wavetable_size as f32;
  let mut accumulator = -1.0;

  let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);
  for _ in 0..wavetable_size {
    wavetable.push(shape_fn(accumulator));
    accumulator += increment;
  }
  
  wavetable
}

fn get_shape_fn(might: Option<Box<dyn Fn(f32)->f32>>) -> Box<dyn Fn(f32)->f32> {
  match might {
    Some(shape_fn) => shape_fn,
    None => Box::new(pure),
  }
}