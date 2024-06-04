#[allow(warnings)]
mod bindings;

use bindings::Guest;

use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref PHASE: Mutex<f32> = Mutex::new(0.0);
    static ref FREQUENCY: Mutex<f32> = Mutex::new(0.0);
    static ref SAMPLE_RATE: Mutex<f32> = Mutex::new(48000.0);
}

struct Component;

impl Guest for Component {
    fn set_freq(freq: f32) {
        *FREQUENCY.lock() = freq;
    }
    fn set_sr(sr: f32) {
        *SAMPLE_RATE.lock() = sr;
    }
    fn process(input: Vec<f32>) -> Vec<f32> {
        let mut phase = PHASE.lock();
        let frequency = *FREQUENCY.lock();
        let sample_rate = *SAMPLE_RATE.lock();
        let delta_phase = frequency / sample_rate;

        let mut output = Vec::with_capacity(input.len());
        // println!("input: {:?}", input);
        for _ in 0..input.len() {
            *phase += delta_phase;
            if *phase > 1.0 {
                *phase -= 1.0;
            }
            output.push((*phase * 2.0 * std::f32::consts::PI).sin());
        }
        output
    }
}

bindings::export!(Component with_types_in bindings);
