pub fn shape(
  wavetable: &mut Vec<f32>,
  shape_fns: Vec<Box<dyn Fn(f32)->f32>>
) -> &mut Vec<f32> {
  shape_fns.iter().for_each(|sf| {
    wavetable.iter_mut().for_each(|s| {
      *s = (*sf)(*s);
    });
  });

  wavetable
}

pub fn gain(gain: f32) -> Box<dyn Fn(f32)->f32> {
  Box::new(move |sample| {
      let gain = f32::from(gain);
      sample * gain
  })
}

pub fn clip(threshold: f32) -> Box<dyn Fn(f32)->f32> {
  Box::new(move |sample| {
    let sign = sample.signum();
    let sample = sample.abs();

    let threshold = f32::from(threshold);
    if sample > threshold {
      threshold * sign
    } else {
      sample * sign
    }
  })
}

pub fn pure(sample: f32) -> f32 {
  sample
}

pub fn square(sample: f32) -> f32 {
  pwm(0.)(sample)
}

pub fn rect(sample: f32) -> f32 {
  pwm(0.8)(sample)
}

pub fn pwm(threshold: f32) -> Box<dyn Fn(f32)->f32> {
  Box::new(move |sample| {
      let width = f32::from(threshold);
      if sample > width {
          1_f32
      } else {
          -1_f32
      }
  })

}

pub fn bit_crush(bits: f32) -> Box<dyn Fn(f32)->f32> {
  Box::new(move |sample: f32| {
    let sign = sample.signum();
    let sample = sample.abs();

    ((sample * bits).floor() / bits) * sign
  })
}

pub fn something(sample: f32) -> f32 {
  let sign = sample.signum();
  let mut sample = sample.abs();
  
  if sample < 0.75 && sample > 0.5 {
    sample = sample * sign * -0.7;
  } else {
    sample = sample * sign 
  }

  sample
}

