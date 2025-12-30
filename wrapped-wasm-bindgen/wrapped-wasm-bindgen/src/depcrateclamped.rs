// Generated macro for Clamped (struct)
macro_rules! DepcrateClamped {
() => {
// Module: crate
// Provides: {"Clamped"}
// Dependencies: {}
# [doc = " A wrapper type around slices and vectors for binding the `Uint8ClampedArray`"] # [doc = " array in JS."] # [doc = ""] # [doc = " If you need to invoke a JS API which must take `Uint8ClampedArray` array,"] # [doc = " then you can define it as taking one of these types:"] # [doc = ""] # [doc = " * `Clamped<&[u8]>`"] # [doc = " * `Clamped<&mut [u8]>`"] # [doc = " * `Clamped<Vec<u8>>`"] # [doc = ""] # [doc = " All of these types will show up as `Uint8ClampedArray` in JS and will have"] # [doc = " different forms of ownership in Rust."] # [derive (Copy , Clone , PartialEq , Debug , Eq)] pub struct Clamped < T > (pub T) ;
};
}
