use crate::utils::create_random_vec_f32;

#[derive(Debug, Clone)]
pub struct NoiseElement {
    occurance: usize, // 1 is every sample, max is sample rate
    random_vec: Vec<f32>,
    random_vec_index: usize,
    sample_rate: usize,
}

impl NoiseElement {
    pub fn new(occurance: usize, min: f32, max: f32, sample_rate: usize) -> NoiseElement {
        NoiseElement {
            occurance,
            random_vec: create_random_vec_f32(min, max, sample_rate),
            random_vec_index: 0,
            sample_rate,
        }
    }
    pub fn next(&mut self, sample_clock: usize) -> f32 {
        if sample_clock % self.occurance == 0 {
            self.random_vec_index = (self.random_vec_index + 1) % self.sample_rate;
            return self.random_vec[self.random_vec_index];
        }
        return 0f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_element_next_returns_f32_in_range() {
        let mock_sample_rate = 100;
        let min = -1.0;
        let max = 1.0;
        let mut noise_element = NoiseElement::new(1, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for _ in 1..mock_sample_rate * 2 {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            assert_ge!(result, min);
            assert_le!(result, max);
        }
    }

    #[test]
    fn noise_element_next_returns_f32_in_range_2() {
        let mock_sample_rate = 100;
        let min = -0.5;
        let max = 0.5;
        let mut noise_element = NoiseElement::new(1, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for _ in 1..mock_sample_rate * 2 {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            assert_ge!(result, min);
            assert_le!(result, max);
        }
    }

    #[test]
    #[should_panic(expected = "max and min must be different")]
    fn noise_element_same_min_max_panics() {
        let mock_sample_rate = 100;
        let min = 1.0;
        let max = 1.0;
        NoiseElement::new(1, min, max, mock_sample_rate);
    }

    #[test]
    #[should_panic(expected = "max must be greater than min")]
    fn noise_element_large_min_panics() {
        let mock_sample_rate = 100;
        let min = 1.0;
        let max = 0.5;
        NoiseElement::new(1, min, max, mock_sample_rate);
    }

    #[test]
    fn noise_element_next_returns_f32_in_range_3() {
        let mock_sample_rate = 100;
        let min = 0.01;
        let max = 1.0;
        let diff = max - min;
        let mut noise_element = NoiseElement::new(1, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for _ in 1..mock_sample_rate * 2 {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            assert_ge!(result, -diff);
            assert_le!(result, diff);
        }
    }

    #[test]
    fn noise_element_next_cycles_through() {
        let mock_sample_rate = 100;
        let mut noise_element = NoiseElement::new(1, -1.0, 1.0, mock_sample_rate);

        let mut sample_clock = 0;
        for _ in 1..mock_sample_rate * 2 {
            sample_clock = sample_clock + 1;
            let _ = noise_element.next(sample_clock);
        };
    }

    #[test]
    fn noise_element_next_occurance_rate_2() {
        let occurance_rate = 2;
        let mock_sample_rate = 100;
        let min = -1.0;
        let max = 1.0;
        let diff = max - min;

        let mut noise_element = NoiseElement::new(occurance_rate, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for i in 0..mock_sample_rate {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            if i % occurance_rate != 0 {
                assert_ge!(result, -diff);
                assert_le!(result, diff);    
            } else {
                assert_eq!(result, 0.0);
            }
        }
    }

    #[test]
    fn noise_element_next_occurance_rate_7() {
        let occurance_rate = 7;

        let mock_sample_rate = 100;
        let min = -1.0;
        let max = 1.0;
        let diff = max - min;

        let mut noise_element = NoiseElement::new(occurance_rate, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for i in 0..mock_sample_rate {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            if i % occurance_rate != 0 {
                assert_ge!(result, -diff);
                assert_le!(result, diff);    
            } else {
                assert_eq!(result, 0.0);
            }
        }
    }
}
