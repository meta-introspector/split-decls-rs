// Generated macro for WasmPrimitive (trait)
macro_rules! Depcrate_convert_traitsWasmPrimitive {
() => {
// Module: crate::convert::traits
// Provides: {"WasmPrimitive"}
// Dependencies: {}
# [doc = " A trait for any type which maps to a Wasm primitive type when used in FFI"] # [doc = " (`i32`, `i64`, `f32`, or `f64`)."] # [doc = ""] # [doc = " This is with the exception of `()` (and other zero-sized types), which are"] # [doc = " also allowed because they're ignored: no arguments actually get added."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is an unsafe trait to implement as there's no guarantee the type"] # [doc = " actually maps to a primitive type."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Unstable"] # [doc = ""] # [doc = " This is part of the internal [`convert`](crate::convert) module, **no"] # [doc = " stability guarantees** are provided. Use at your own risk. See its"] # [doc = " documentation for more details."] pub unsafe trait WasmPrimitive : Default { }
};
}
