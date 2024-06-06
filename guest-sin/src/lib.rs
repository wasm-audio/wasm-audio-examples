mod prelude;
use prelude::*;

#[allow(warnings)]
mod bindings;

use bindings::Guest;

def_param!(PHASE, 0.0);
def_param!(FREQUENCY, 440.0);
def_param!(SAMPLE_RATE, 48000.0);

pub struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        match key.as_str() {
            "phase" => set_param!(PHASE, value),
            "frequency" => set_param!(FREQUENCY, value),
            "sample_rate" => set_param!(SAMPLE_RATE, value),
            _ => (),
        }
    }

    fn process(input: Vec<f32>) -> Vec<f32> {
        let frequency = get_param!(FREQUENCY);
        let sample_rate = get_param!(SAMPLE_RATE);
        let mut phase = get_param!(PHASE);
        let delta_phase = frequency / sample_rate;

        let mut output = Vec::with_capacity(input.len());
        for _ in 0..input.len() {
            phase += delta_phase;
            if phase > 1.0 {
                phase -= 1.0;
            }
            output.push((phase * 2.0 * std::f32::consts::PI).sin());
        }

        set_param!(PHASE, phase);

        output
    }
}

bindings::export!(Component with_types_in bindings);
