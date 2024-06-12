pub use std::cell::Cell;

#[macro_export]
macro_rules! init_param {
    ($name:ident, $value:expr) => {
        thread_local! {
            pub static $name: Cell<f32> = Cell::new($value);
        }
    };
}

#[macro_export]
macro_rules! set_param {
    ($name:ident, $value:expr) => {
        $name.with(|param| param.set($value))
    };
}

#[macro_export]
macro_rules! get_param {
    ($name:ident) => {
        $name.with(|param| param.get())
    };
}
