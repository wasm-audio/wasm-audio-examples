#[allow(warnings)]
mod bindings;
use bindings::*;
use glicol::Engine;
use wasm_audio_utils::*;

init_param!(T1_AMP, f32, 1.0);
init_param!(T2_AMP, f32, 0.6);
init_param!(T3_AMP, f32, 0.05);
init_param!(BPM, f32, 120.0);

init_param!(CODE, String, include_str!("techno.glicol").into());
init_param!(ENGINE, Engine<128>, {
    let mut engine = Engine::<128>::new();
    let code = make_code();
    engine.update_with_code(&code);
    engine.set_sr(48000);
    engine.livecoding = false;
    engine
});

fn make_code() -> String {
    CODE.with(|code| {
        let code = code.borrow();
        code.replace("$t1_amp", &get_param!(T1_AMP).to_string())
            .replace("$t2_amp", &get_param!(T2_AMP).to_string())
            .replace("$t3_amp", &get_param!(T3_AMP).to_string())
    })
}

struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        ENGINE.with(|engine| {
            let mut engine = engine.borrow_mut();
            match key.as_str() {
                "sample_rate" | "sr" => engine.set_sr(value as usize),
                "bpm" => engine.set_bpm(value),
                "t1_amp" => {
                    set_param!(T1_AMP, value);
                    let code = make_code();
                    engine.update_with_code(&code);
                }
                "t2_amp" => {
                    set_param!(T2_AMP, value);
                    let code = make_code();
                    engine.update_with_code(&code);
                }
                "t3_amp" => {
                    set_param!(T3_AMP, value);
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
        ENGINE.with(|engine| {
            let mut engine = engine.borrow_mut();
            let block_size = input.len();
            let round = block_size / 128;
            let mut output = Vec::with_capacity(input.len());
            for _ in 0..round {
                let (buffer, _result) = engine.next_block(vec![]);
                output.extend(buffer[0].iter());
            }
            output
        })
    }
}

bindings::export!(Component with_types_in bindings);
