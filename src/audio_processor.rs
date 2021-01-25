use crate::noise_element::NoiseElement;
use crate::utils::*;
use std::sync::Arc;
use std::sync::Mutex;
use web_sys::console::log;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct AudioProcessor {
    sample_clock: usize,
    sample_rate: usize,
    noise_element_1: NoiseElement,
    noise_element_2: NoiseElement,
    noise_element_3: NoiseElement,
    pink_noise_freq_1: f32,
    pink_noise_freq_2: f32,
    pink_noise_freq_3: f32,
    pink_noise_freq_4: f32,
    pink_noise_freq_5: f32,
    seconds_gone_by: usize,
}

impl AudioProcessor {
    pub fn new(sample_rate: usize) -> AudioProcessor {
        AudioProcessor {
            sample_clock: 0,
            sample_rate,
            noise_element_1: NoiseElement::new(5, -1.0, 1.0, sample_rate),
            noise_element_2: NoiseElement::new(4, -0.3, 0.5, sample_rate),
            noise_element_3: NoiseElement::new(3, -0.5, 0.3, sample_rate),
            pink_noise_freq_1: 0.0,
            pink_noise_freq_2: 0.0,
            pink_noise_freq_3: 0.0,
            pink_noise_freq_4: 0.0,
            pink_noise_freq_5: 0.0,
            seconds_gone_by: 0,
        }
    }

    fn increment_sample_clock(&mut self) {
        let incremented_sample_clock = self.sample_clock + 1;

        if incremented_sample_clock == self.sample_rate {
            self.seconds_gone_by = self.seconds_gone_by + 1;            
        }

        self.sample_clock = incremented_sample_clock % self.sample_rate;
    }

    fn sine(&mut self, freq: f32) -> f32 {
        ((self.sample_clock as f32) * freq * 2.0 * std::f32::consts::PI / (self.sample_rate as f32))
            .sin()
    }

    // we will sines of lower frequencies and hope this words
    pub fn pink_noise(&mut self) -> f32 {
        self.increment_sample_clock();
        let sample_rate_f32 = self.sample_rate as f32;


        let mut final_f32 = 0.0;

        let freq_count = 10;
        let freq_count_f32 = freq_count as f32;

        for i in 1..=freq_count {
            let i_f32 = i as f32;
            let freq = i_f32 / freq_count_f32 * sample_rate_f32 / freq_count_f32;
            // low have higher amp
            let amp = 1.0 - ((i_f32 - 1.0) / i_f32);
            let sine = self.sine(freq) * amp;
            final_f32 = final_f32 + sine;
        }

        final_f32
        // change low least
        // if self.sample_clock % 880 == 0 {
        //     self.pink_noise_freq_1 =  440.0 + (random_f32() * 440.0);
        // }
        // if self.sample_clock % 2000 == 0 {
        //     self.pink_noise_freq_2 =  400.0 + random_f32() * 400.0;
        // }
        // if self.sample_clock % 80 == 0 {
        //     self.pink_noise_freq_2 = random_f32() * 800.0;
        // }
        // if self.sample_clock % 60 == 0 {
        //     self.pink_noise_freq_3 = random_f32() * 1600.0;
        // }

        // let sine_1 = self.sine(self.pink_noise_freq_1) * 0.1;
        // let sine_2 = self.sine(self.pink_noise_freq_2) * 0.1;
        // let sine_3 = self.sine(self.pink_noise_freq_3);

        // (sine_1 + sine_2 + sine_3) / 3.0
        // (sine_1 + sine_2) / 2.0;
        // sine_1
    }

    pub fn colored_noise(&mut self, fade_in: f32, sustain: f32, fade_out: f32) -> f32 {
        self.increment_sample_clock();

        let sample_clock = self.sample_clock as f32;
        let sample_rate = self.sample_rate as f32;
        let seconds_gone_by = self.seconds_gone_by as f32;

        let mut amp = 0.0;
        if seconds_gone_by < fade_in {
            amp = get_amp_for_fade_in(fade_in, sample_rate, seconds_gone_by, sample_clock);
        } else if seconds_gone_by < fade_in + sustain {
            amp = 1.0;
        } else if seconds_gone_by < fade_in + sustain + fade_out {
            let relative_seconds_gone_by = seconds_gone_by - fade_in - sustain;
            amp = get_amp_for_fade_out(fade_out, sample_rate, relative_seconds_gone_by, sample_clock);
        } else {
            amp = 0.0
        }

        let noise_element_1 = self.noise_element_1.next(self.sample_clock);
        let noise_element_2 = self.noise_element_2.next(self.sample_clock);
        let noise_element_3 = self.noise_element_3.next(self.sample_clock);

        return (noise_element_1 + noise_element_2 + noise_element_3) * (1.0/3.0) * amp
    }
}

pub type AudioProcessorHandle = Arc<Mutex<AudioProcessor>>;

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
}