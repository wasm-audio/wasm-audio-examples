#[allow(warnings)]
mod bindings;

use std::vec;

use bindings::*;

use glicol::Engine;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod prelude;
use prelude::*;

init_param!(T1_AMP, 1.0);
init_param!(T2_AMP, 0.6);
init_param!(T3_AMP, 0.05);
init_param!(BPM, 120.0);

lazy_static! {
    static ref CODE: Mutex<String> = Mutex::new(include_str!("techno.glicol").into());
    static ref ENGINE: Mutex<Engine<128>> = Mutex::new({
        let mut engine = Engine::new();
        engine.update_with_code(
            &CODE
                .lock()
                .unwrap()
                .replace("$t1_amp", get_param!(T1_AMP).to_string().as_str())
                .replace("$t2_amp", get_param!(T2_AMP).to_string().as_str())
                .replace("$t3_amp", get_param!(T3_AMP).to_string().as_str()),
        );
        engine.set_sr(48000);
        engine.livecoding = false;
        engine
    });
}

struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        let mut engine = ENGINE.lock().unwrap();
        match key.as_str() {
            "sample_rate" | "sr" => engine.set_sr(value as usize),
            "bpm" => engine.set_bpm(value),
            "t1_amp" => {
                set_param!(T1_AMP, value);
                engine.update_with_code(
                    &CODE
                        .lock()
                        .unwrap()
                        .replace("$t1_amp", get_param!(T1_AMP).to_string().as_str())
                        .replace("$t2_amp", get_param!(T2_AMP).to_string().as_str())
                        .replace("$t3_amp", get_param!(T3_AMP).to_string().as_str()),
                );
            }
            "t2_amp" => {
                set_param!(T2_AMP, value);
                let code = CODE
                    .lock()
                    .unwrap()
                    .replace("$t1_amp", get_param!(T1_AMP).to_string().as_str())
                    .replace("$t2_amp", get_param!(T2_AMP).to_string().as_str())
                    .replace("$t3_amp", get_param!(T3_AMP).to_string().as_str());
                engine.update_with_code(&code);
            }
            "t3_amp" => {
                set_param!(T3_AMP, value);
                engine.update_with_code(
                    &CODE
                        .lock()
                        .unwrap()
                        .replace("$t1_amp", get_param!(T1_AMP).to_string().as_str())
                        .replace("$t2_amp", get_param!(T2_AMP).to_string().as_str())
                        .replace("$t3_amp", get_param!(T3_AMP).to_string().as_str()),
                );
            }

            _ => (),
        }
    }

    fn get_params() -> Vec<ParamInfo> {
        vec![
            ParamInfo {
                name: "bpm".into(),
                min: 40.0,
                max: 360.0,
                default: 120.0,
            },
            ParamInfo {
                name: "t1_amp".into(),
                min: 0.0,
                max: 1.0,
                default: 1.0,
            },
            ParamInfo {
                name: "t2_amp".into(),
                min: 0.0,
                max: 1.0,
                default: 0.6,
            },
            ParamInfo {
                name: "t3_amp".into(),
                min: 0.0,
                max: 1.0,
                default: 0.05,
            },
        ]
    }

    fn process(input: Vec<f32>) -> Vec<f32> {
        let mut engine = ENGINE.lock().unwrap();
        let block_size = input.len();
        let round = block_size / 128;
        let mut output = Vec::with_capacity(input.len());
        for _ in 0..round {
            let (buffer, _result) = engine.next_block(vec![]);
            output.extend(buffer[0].iter());
        }
        output
    }
}

bindings::export!(Component with_types_in bindings);
