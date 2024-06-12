#[allow(warnings)]
mod bindings;

use bindings::Guest;
use bindings::ParamInfo;

use wasm_audio_utils::*;

init_param!(PHASE, 0.0);
init_param!(FREQUENCY, 440.0);
init_param!(SAMPLE_RATE, 48000.0);
init_param!(AMPLITUDE, 1.0);

pub struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        match key.as_str() {
            "phase" => set_param!(PHASE, value),
            "frequency" | "freq" => set_param!(FREQUENCY, value),
            "sample_rate" | "sr" => set_param!(SAMPLE_RATE, value),
            "amplitude" | "amp" => set_param!(AMPLITUDE, value),
            _ => println!("Unknown parameter: {}", key),
        }
    }

    fn get_params() -> Vec<ParamInfo> {
        return vec![
            ParamInfo {
                name: "frequency".to_string(),
                min: 20.0,
                max: 20000.0,
                default: 440.0,
            },
            ParamInfo {
                name: "amplitude".to_string(),
                min: 0.0,
                max: 1.0,
                default: 1.0,
            },
        ];
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
            output.push((phase * 2.0 * std::f32::consts::PI).sin() * get_param!(AMPLITUDE));
        }

        set_param!(PHASE, phase);

        output
    }
}

bindings::export!(Component with_types_in bindings);
