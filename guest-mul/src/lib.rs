#[allow(warnings)]
mod bindings;

use bindings::Guest;

use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref FACTOR: Mutex<f32> = Mutex::new(1.0);
}

struct Component;

impl Guest for Component {
    fn set_factor(f: f32) {
        *FACTOR.lock() = f;
    }
    fn process(input: Vec<f32>) -> Vec<f32> {
        let mut output = Vec::with_capacity(input.len());
        // println!("input: {:?}", input);
        for v in input {
            let f = *FACTOR.lock();
            output.push(v * f);
        }
        output
    }
}

bindings::export!(Component with_types_in bindings);
