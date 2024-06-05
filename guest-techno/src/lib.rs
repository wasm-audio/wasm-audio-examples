#[allow(warnings)]
mod bindings;

use bindings::Guest;

use glicol::Engine;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref ENGINE: Mutex<Engine<128>> = Mutex::new(Engine::<128>::new());
}

struct Component;

impl Guest for Component {
    fn set_sr(sr: f32) {
        let mut engine = ENGINE.lock();
        engine.set_sr(sr as usize);
    }

    fn set_code(code: String) {
        let mut engine = ENGINE.lock();
        engine.update_with_code(&code);
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
