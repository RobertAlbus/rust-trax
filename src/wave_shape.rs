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
      let mut sample = (sample + 2.) / 2.;
      sample = (sample * bits).floor() / (bits -1.);
      sample = sample * 2. - 2.;
      
      sample
  })
}

pub fn something(sample: f32) -> f32 {
  let sign = sample.signum();
  let mut sample = sample.abs();
  
  if sample < 0.55 && sample > 0.5 {
    sample = sample * sign * -1.;
  } else {
    sample = sample * sign 
  }

  sample
}

