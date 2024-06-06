#[allow(warnings)]
mod bindings;

use bindings::Guest;

use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::Mutex;

pub type Parameters = HashMap<String, f32>;

lazy_static! {
    // static ref PHASE: Mutex<f32> = Mutex::new(0.0);
    // static ref FREQUENCY: Mutex<f32> = Mutex::new(440.0);
    // static ref SAMPLE_RATE: Mutex<f32> = Mutex::new(48000.0);
    static ref PARAMETES: Mutex<Parameters> = Mutex::new(Parameters::from_iter(
        [
            ("phase".to_string(), 0.0),
            ("frequency".to_string(), 440.0),
            ("sample_rate".to_string(), 48000.0),
        ]
    ));
}

struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        let mut parameters = PARAMETES.lock();
        parameters.insert(key, value);
    }
    fn process(input: Vec<f32>) -> Vec<f32> {
        let mut parameters = PARAMETES.lock();

        let frequency = *parameters.get_mut("frequency").unwrap();
        let sample_rate = *parameters.get_mut("sample_rate").unwrap();
        let mut phase = *parameters.get_mut("phase").unwrap();
        let delta_phase = frequency / sample_rate;

        let mut output = Vec::with_capacity(input.len());
        // println!("input: {:?}", input);
        for _ in 0..input.len() {
            phase += delta_phase;
            if phase > 1.0 {
                phase -= 1.0;
            }
            parameters.insert("phase".to_string(), phase);
            output.push((phase * 2.0 * std::f32::consts::PI).sin());
        }
        output
    }
}

bindings::export!(Component with_types_in bindings);
