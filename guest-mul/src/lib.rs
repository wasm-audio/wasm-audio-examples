#[allow(warnings)]
mod bindings;

use crate::bindings::ParamInfo;
use bindings::Guest;
use wasm_audio_utils::*;

init_param!(FACTOR, 1.0);

struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        match key.as_str() {
            "factor" => set_param!(FACTOR, value),
            _ => (),
        }
    }
    fn get_params() -> Vec<ParamInfo> {
        return vec![ParamInfo {
            name: "factor".to_string(),
            min: 0.0,
            max: 1.0,
            default: 1.0,
        }];
    }
    fn process(input: Vec<f32>) -> Vec<f32> {
        input.iter().map(|v| v * get_param!(FACTOR)).collect()
    }
}

bindings::export!(Component with_types_in bindings);
