#[allow(warnings)]
mod bindings;

use bindings::*;
use glicol::Engine;
use std::cell::RefCell;
use wasm_audio_utils::*;

init_param!(BANDWIDTH, f32, 0.7);
init_param!(DAMPING, f32, 0.1);
init_param!(DECAY, f32, 0.3);
init_param!(MIX, f32, 0.1);

thread_local! {
    static CODE: RefCell<String> = RefCell::new(include_str!("reverb.glicol").into());
    static ENGINE: RefCell<Engine<128>> = RefCell::new({
        let mut engine = Engine::new();
        let code = make_code();
        engine.update_with_code(&code);
        engine.set_sr(48000);
        engine.livecoding = false;
        engine
    });
}

fn make_code() -> String {
    CODE.with(|code| {
        let code = code.borrow();
        code.replace("#bandwidth", &get_param!(BANDWIDTH).to_string())
            .replace("#damping", &get_param!(DAMPING).to_string())
            .replace("#decay", &get_param!(DECAY).to_string())
            .replace("#wetmix", &get_param!(MIX).to_string())
            .replace("#drymix", &(1.0 - get_param!(MIX)).to_string())
    })
}

struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        ENGINE.with(|engine| {
            let mut engine = engine.borrow_mut();
            match key.as_str() {
                "sample_rate" | "sr" => engine.set_sr(value as usize),
                "bandwidth" => {
                    set_param!(BANDWIDTH, value);
                    let code = make_code();
                    engine.update_with_code(&code);
                }
                "damping" => {
                    set_param!(DAMPING, value);
                    let code = make_code();
                    engine.update_with_code(&code);
                }
                "decay" => {
                    set_param!(DECAY, value);
                    let code = make_code();
                    engine.update_with_code(&code);
                }
                "mix" => {
                    set_param!(MIX, value);
                    let code = make_code();
                    engine.update_with_code(&code);
                }
                _ => (),
            }
        });
    }

    fn get_params() -> Vec<ParamInfo> {
        vec![
            ParamInfo {
                name: "bandwidth".into(),
                min: 0.0,
                max: 1.0,
                default: get_param!(BANDWIDTH),
            },
            ParamInfo {
                name: "damping".into(),
                min: 0.0,
                max: 1.0,
                default: get_param!(DAMPING),
            },
            ParamInfo {
                name: "decay".into(),
                min: 0.0,
                max: 0.9999,
                default: get_param!(DECAY),
            },
            ParamInfo {
                name: "mix".into(),
                min: 0.0,
                max: 1.0,
                default: get_param!(MIX),
            },
        ]
    }

    fn process(input: Vec<f32>) -> Vec<f32> {
        ENGINE.with(|engine| {
            let mut engine = engine.borrow_mut();
            let block_size = input.len();
            let round = block_size / 128;
            let mut output = Vec::with_capacity(input.len());

            for r in 0..round {
                let input_left = &input[r * 128..(r + 1) * 128];
                let inpbuf = vec![input_left];
                let (buffer, _result) = engine.next_block(inpbuf);
                output.extend(buffer[0].iter());
            }
            output
        })
    }
}

bindings::export!(Component with_types_in bindings);
