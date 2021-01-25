use getrandom::getrandom;

// relative_seconds_gone_by is 0 when fade in starts and fade_out_length when it's done
pub fn clamp(num: f32) -> f32 {
  if num > 1.0 {
      return 1.0;
  }
  if num < -1.0 {
      return -1.0;
  }
  num
}

// Creates a random f32 vector with values between -1 and 1
pub fn create_random_vec_f32(min: f32, max: f32, size: usize) -> Vec<f32> {
  if max == min {
      panic!("max and min must be different")
  } else if max < min {
      panic!("max must be greater than min")
  }

  // u8 is between 0 and 255
  // buffer size can't be set dynamically....
  let mut buff: Vec<u8> = Vec::with_capacity(size);
  for _ in 0..size {
      buff.push(0);
  }
  getrandom(&mut buff).unwrap();

  let diff = max - min;
  // convert to floats
  buff.iter()
      .map(|n| (*n as f32) / 255.0 * diff - diff / 2.0)
      // .map(|f| (f - min) / (max - min))
      .collect()
}

pub fn random_f32() -> f32 {
  // u8 is between 0 and 255
  // buffer size can't be set dynamically....
  let mut buff = [0u8; 3];
  getrandom(&mut buff).unwrap();

  // between 0 and 765
  let max = 3 * 255;
  let sum: i32 = buff.iter().map(|n| (*n as i32)).sum();

  // betwen 0.0 and 1.0
  (sum as f32) / (max as f32)
}

pub fn get_amp_for_fade_out(fade_out_length: f32, sample_rate: f32, relative_seconds_gone_by: f32, sample_clock: f32) -> f32 {
  let current_sec_amp = get_amp_for_fade_out_at_second(fade_out_length, relative_seconds_gone_by);
  let next_sec_amp = get_amp_for_fade_out_at_second(fade_out_length, relative_seconds_gone_by + 1.0);

  let percent_complete = sample_clock / sample_rate;
  let inverse = 1.0 - percent_complete;
  // we start at the current_sec_amp and we end at the next_second_amp
  return current_sec_amp * inverse + next_sec_amp * percent_complete;
}

// start at 1.0 and end at 0.0
pub fn get_amp_for_fade_out_at_second(fade_out_length: f32, relative_seconds_gone_by: f32) -> f32 {
  if (relative_seconds_gone_by <= 0.0) {
      return 1.0;
  }

  if relative_seconds_gone_by >= fade_out_length {
      return 0.0;
  }

  return 1.0 - (relative_seconds_gone_by / fade_out_length);
}

// relative_seconds_gone_by is 0 when fade in starts and fade_out_length when it's done
pub fn get_amp_for_fade_in(fade_in_length: f32, sample_rate: f32, relative_seconds_gone_by: f32, sample_clock: f32) -> f32 {
  let current_sec_amp = get_amp_for_fade_in_at_second(fade_in_length, relative_seconds_gone_by);
  let next_sec_amp = get_amp_for_fade_in_at_second(fade_in_length, relative_seconds_gone_by + 1.0);

  let percent_complete = sample_clock / sample_rate;
  let inverse = 1.0 - percent_complete;
  // we start at the current_sec_amp and we end at the next_second_amp
  return current_sec_amp * inverse + next_sec_amp * percent_complete;
}

// start at 0.0 and end at 1.0
pub fn get_amp_for_fade_in_at_second(fade_out_length: f32, relative_seconds_gone_by: f32) -> f32 {
  if relative_seconds_gone_by <= 0.0 {
      return 0.0;
  }

  if relative_seconds_gone_by >= fade_out_length {
      return 1.0;
  }

  return relative_seconds_gone_by / fade_out_length;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_amp_for_fade_out_at_second_initial() {
        let result = get_amp_for_fade_out_at_second(10.0, 0.0);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn get_amp_for_fade_out_at_second_half() {
        let result = get_amp_for_fade_out_at_second(10.0, 5.0);

        assert_eq!(result, 0.5);
    }

    #[test]
    fn get_amp_for_fade_out_at_second_end() {
        let result = get_amp_for_fade_out_at_second(10.0, 10.0);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn get_amp_for_fade_out_initial_is_1() {
        let result = get_amp_for_fade_out(2.0, 100.0, 0.0, 0.0);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn get_amp_for_fade_out_half_is_half() {
        let result = get_amp_for_fade_out(2.0, 100.0, 1.0, 0.0);

        assert_eq!(result, 0.5);
    }

    #[test]
    fn get_amp_for_fade_out_between_sec_1_and_2() {
        let result = get_amp_for_fade_out(2.0, 100.0, 1.0, 50.0);

        assert_eq!(result, 0.25);
    }

    #[test]
    fn get_amp_for_fade_out_end_sec_1_greater_than_half() {
        let result = get_amp_for_fade_out(2.0, 100.0, 0.0, 99.0);

        assert_gt!(result, 0.5);
    }

    #[test]
    fn create_random_vec_f32_is_correct_size() {
        let mock_size = 400;
        let random_vec = create_random_vec_f32(-1.0, 1.0, mock_size);

        assert_eq!(random_vec.len(), mock_size);
    }

    #[test]
    fn create_random_vec_f32_no_elements_less_than_min() {
        let random_vec = create_random_vec_f32(-1.0, 1.0, 99);

        let less_than_min: Vec<f32> = random_vec
            .clone()
            .into_iter()
            .filter(|n| n < &-1.0)
            .collect();

        assert_eq!(less_than_min.len(), 0);
    }

    #[test]
    fn create_random_vec_f32_no_elements_greater_than_max() {
        let random_vec = create_random_vec_f32(-1.0, 1.0, 99);

        let greater_than_max: Vec<f32> = random_vec
            .clone()
            .into_iter()
            .filter(|n| n > &1.0)
            .collect();

        assert_eq!(greater_than_max.len(), 0);
    }

    #[test]
    fn clamp_works() {
        let result_high = clamp(1.5);
        assert_eq!(result_high, 1.0);

        let result_low = clamp(-4.5);
        assert_eq!(result_low, -1.0);

        let result_pass = clamp(0.5);
        assert_eq!(result_pass, 0.5);
    }
}
