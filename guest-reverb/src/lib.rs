#[allow(warnings)]
mod bindings;

use std::vec;

use bindings::*;

use glicol::Engine;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod prelude;
use prelude::*;

init_param!(BANDWIDTH, 0.7);
init_param!(DAMPING, 0.1);
init_param!(DECAY, 0.3);
init_param!(MIX, 0.1);

lazy_static! {
    static ref CODE: Mutex<String> = Mutex::new(include_str!("reverb.glicol").into());
    static ref ENGINE: Mutex<Engine<128>> = Mutex::new({
        let mut engine = Engine::new();
        engine.update_with_code(
            &CODE
                .lock()
                .unwrap()
                .replace("#bandwidth", get_param!(BANDWIDTH).to_string().as_str())
                .replace("#damping", get_param!(DAMPING).to_string().as_str())
                .replace("#decay", get_param!(DAMPING).to_string().as_str())
                .replace("#wetmix", get_param!(MIX).to_string().as_str())
                .replace("#drymix", (1.0 - get_param!(MIX)).to_string().as_str()),
        );
        engine.set_sr(48000);
        engine.livecoding = false;
        engine
    });
}

#[macro_export]
macro_rules! update_engine {
    (
        $engine:ident
    ) => {
        $engine.update_with_code(
            &CODE
                .lock()
                .unwrap()
                .replace("#bandwidth", get_param!(BANDWIDTH).to_string().as_str())
                .replace("#damping", get_param!(DAMPING).to_string().as_str())
                .replace("#decay", get_param!(DAMPING).to_string().as_str())
                .replace("#wetmix", get_param!(MIX).to_string().as_str())
                .replace("#drymix", (1.0 - get_param!(MIX)).to_string().as_str()),
        );
    };
}

struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        let mut engine = ENGINE.lock().unwrap();
        match key.as_str() {
            "sample_rate" | "sr" => engine.set_sr(value as usize),
            "bandwidth" => {
                set_param!(BANDWIDTH, value);
                update_engine!(engine);
            }
            "damping" => {
                set_param!(DAMPING, value);
                update_engine!(engine);
            }
            "decay" => {
                set_param!(DECAY, value);
                update_engine!(engine);
            }
            "mix" => {
                set_param!(MIX, value);
                update_engine!(engine);
            }
            _ => (),
        }
    }

    fn get_params() -> Vec<ParamInfo> {
        vec![
            ParamInfo {
                name: "bandwidth".to_string(),
                min: 0.0,
                max: 1.0,
                default: get_param!(BANDWIDTH),
            },
            ParamInfo {
                name: "damping".to_string(),
                min: 0.0,
                max: 1.0,
                default: get_param!(DAMPING),
            },
            ParamInfo {
                name: "decay".to_string(),
                min: 0.0,
                max: 0.9999,
                default: get_param!(DECAY),
            },
            ParamInfo {
                name: "mix".to_string(),
                min: 0.0,
                max: 1.0,
                default: get_param!(MIX),
            },
        ]
    }

    fn process(input: Vec<f32>) -> Vec<f32> {
        let mut engine = ENGINE.lock().unwrap();
        let block_size = input.len();
        let round = block_size / 128;
        let mut output = Vec::with_capacity(input.len());

        for r in 0..round {
            let input_left = &input[r * 128..(r + 1) * 128];
            // let input_right = &input[block_size / 2 + r * 128..block_size / 2 + (r + 1) * 128];
            let inpbuf = vec![input_left];
            let (buffer, _result) = engine.next_block(inpbuf);
            output.extend(buffer[0].iter());
        }
        output
    }
}

bindings::export!(Component with_types_in bindings);
