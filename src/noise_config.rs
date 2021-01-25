#[derive(Debug, Copy, Clone)]
pub enum NoiseType {
    Default,
}

fn convert_string_to_noise_type(noise_type: String) -> NoiseType {
    match noise_type.as_str() {
        "default" => NoiseType::Default,
        _ => panic!("Unkown NoiseType {}", noise_type),
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NoiseConfig {
  pub noise_type: NoiseType,
  pub fade_in: f32,
  pub hold: f32,
  pub fade_out: f32
}

impl NoiseConfig {
  pub fn new(noise_type: String, fade_in: f32, hold: f32, fade_out: f32) -> NoiseConfig {
      NoiseConfig {
          noise_type: convert_string_to_noise_type(noise_type),
          fade_in,
          hold,
          fade_out,
      }
  }
}
