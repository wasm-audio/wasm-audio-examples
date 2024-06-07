pub use atomic_float::AtomicF32;
pub use lazy_static::lazy_static;
pub use std::sync::atomic::Ordering;

#[macro_export]
macro_rules! init_param {
    ($name:ident, $value:expr) => {
        lazy_static! {
            static ref $name: AtomicF32 = AtomicF32::new($value);
        }
    };
}

#[macro_export]
macro_rules! set_param {
    ($name:ident, $value:expr) => {
        $name.store($value, Ordering::SeqCst)
    };
}

#[macro_export]
macro_rules! get_param {
    ($name:ident) => {
        $name.load(Ordering::SeqCst)
    };
}
