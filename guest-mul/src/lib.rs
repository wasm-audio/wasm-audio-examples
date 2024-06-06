#[allow(warnings)]
mod bindings;

use bindings::Guest;

mod prelude;
use prelude::*;

def_param!(FACTOR, 1.0);

struct Component;

impl Guest for Component {
    fn set(key: String, value: f32) {
        match key.as_str() {
            "factor" => set_param!(FACTOR, value),
            _ => (),
        }
    }
    fn process(input: Vec<f32>) -> Vec<f32> {
        input.iter().map(|v| v * get_param!(FACTOR)).collect()
    }
}

bindings::export!(Component with_types_in bindings);
