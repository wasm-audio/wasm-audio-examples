#[allow(warnings)]
mod bindings;

use std::vec;

use bindings::*;

use glicol::Engine;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref ENGINE: Mutex<Engine<128>> = Mutex::new({
        let mut engine = Engine::new();
        engine.update_with_code(
            "~osc1: saw ~pitch
~osc2: squ ~pitch

~env: ~seq >> envperc 0.001 0.1 >> mul 1.0

~seq: speed 2.0 >> seq 60 _60 _60 60
>> mul 0.30

~pitch: ~seq >> mul 261.3

~t1: mix ~osc.. >> lpf 300.0 0.33 >> mul ~env
>> mul 1.5

o: mix ~t.. >> mul 1 >> plate 0.2

~t2: speed 4.0 >> seq _ 60 >> bd 0.2 >> mul 0.9

~t3: speed 4.0 >> seq 60 61 63 62 >> hh 0.02 >> mul 0.05"
                .into(),
        );
        engine
    });
}

struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        let mut engine = ENGINE.lock();
        match key.as_str() {
            "sample_rate" | "sr" => engine.set_sr(value as usize),
            "bpm" => engine.set_bpm(value),
            _ => (),
        }
    }

    fn set_code(code: String) {
        let mut engine = ENGINE.lock();
        engine.update_with_code(&code);
    }

    fn get_params() -> Vec<ParamInfo> {
        vec![ParamInfo {
            name: "bpm".into(),
            min: 40.0,
            max: 360.0,
            default: 120.0,
        }]
    }

    fn process(input: Vec<f32>) -> Vec<f32> {
        let mut engine = ENGINE.lock();
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
