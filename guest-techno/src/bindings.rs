// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_set_sr_cabi<T: Guest>(arg0: f32) {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    T::set_sr(arg0);
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_set_code_cabi<T: Guest>(arg0: *mut u8, arg1: usize) {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    T::set_code(_rt::string_lift(bytes0));
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_process_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let len0 = arg1;
    let result1 = T::process(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let vec3 = (result1).into_boxed_slice();
    let ptr3 = vec3.as_ptr().cast::<u8>();
    let len3 = vec3.len();
    ::core::mem::forget(vec3);
    *ptr2.add(4).cast::<usize>() = len3;
    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_process<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    let base2 = l0;
    let len2 = l1;
    _rt::cabi_dealloc(base2, len2 * 4, 4);
}
pub trait Guest {
    fn set_sr(sr: f32);
    fn set_code(code: _rt::String);
    fn process(input: _rt::Vec<f32>) -> _rt::Vec<f32>;
}
#[doc(hidden)]

macro_rules! __export_world_audio_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "set-sr"]
    unsafe extern "C" fn export_set_sr(arg0: f32,) {
      $($path_to_types)*::_export_set_sr_cabi::<$ty>(arg0)
    }
    #[export_name = "set-code"]
    unsafe extern "C" fn export_set_code(arg0: *mut u8,arg1: usize,) {
      $($path_to_types)*::_export_set_code_cabi::<$ty>(arg0, arg1)
    }
    #[export_name = "process"]
    unsafe extern "C" fn export_process(arg0: *mut u8,arg1: usize,) -> *mut u8 {
      $($path_to_types)*::_export_process_cabi::<$ty>(arg0, arg1)
    }
    #[export_name = "cabi_post_process"]
    unsafe extern "C" fn _post_return_process(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_process::<$ty>(arg0)
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_audio_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
mod _rt {

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::string::String;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_audio_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_audio_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_audio_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:audio:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 224] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07e\x01A\x02\x01A\x07\x01\
@\x01\x02srv\x01\0\x04\0\x06set-sr\x01\0\x01@\x01\x04codes\x01\0\x04\0\x08set-co\
de\x01\x01\x01pv\x01@\x01\x05input\x02\0\x02\x04\0\x07process\x01\x03\x04\x01\x13\
component:sin/audio\x04\0\x0b\x0b\x01\0\x05audio\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}