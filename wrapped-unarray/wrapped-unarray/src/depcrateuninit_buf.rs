// Generated macro for uninit_buf (function)
macro_rules! Depcrateuninit_buf {
() => {
// Module: crate
// Provides: {"uninit_buf"}
// Dependencies: {}
# [doc = " Create an array of unintialized memory"] # [doc = ""] # [doc = " This function is just a safe wrapper around `MaybeUninit::uninit().assume_init()`, which is"] # [doc = " safe when used to create a `[MaybeUninit<T>; N]`, since this type explicitly requires no"] # [doc = " initialization"] # [doc = ""] # [doc = " ```"] # [doc = " # use unarray::*;"] # [doc = " let mut buffer = uninit_buf::<i32, 1000>();"] # [doc = ""] # [doc = " for elem in &mut buffer {"] # [doc = "   elem.write(123);"] # [doc = " }"] # [doc = ""] # [doc = " let result = unsafe { mark_initialized(buffer) };"] # [doc = " assert_eq!(result, [123; 1000])"] # [doc = " ```"] # [doc = ""] # [doc = " This is similar to the nightly-only [`core::mem::MaybeUninit::uninit_array`]"] pub fn uninit_buf < T , const N : usize > () -> [MaybeUninit < T > ; N] { unsafe { MaybeUninit :: uninit () . assume_init () } }
};
}
