// Generated macro for _ (const)
macro_rules! Depcrate_ {
() => {
// Module: crate
// Provides: {"_"}
// Dependencies: {}
const _ : () = { # [doc = " Dummy empty function provided in order to detect linker-injected functions like `__wasm_call_ctors` and others that should be skipped by the wasm-bindgen interpreter."] # [doc = ""] # [doc = " ## About `__wasm_call_ctors`"] # [doc = ""] # [doc = " There are several ways `__wasm_call_ctors` is introduced by the linker:"] # [doc = ""] # [doc = " * Using `#[link_section = \".init_array\"]`;"] # [doc = " * Linking with a C library that uses `__attribute__((constructor))`."] # [doc = ""] # [doc = " The Wasm linker will insert a call to the `__wasm_call_ctors` function at the beginning of every"] # [doc = " function that your module exports if it regards a module as having \"command-style linkage\"."] # [doc = " Specifically, it regards a module as having \"command-style linkage\" if:"] # [doc = ""] # [doc = " * it is not relocatable;"] # [doc = " * it is not a position-independent executable;"] # [doc = " * and it does not call `__wasm_call_ctors`, directly or indirectly, from any"] # [doc = "   exported function."] # [no_mangle] pub extern "C" fn __wbindgen_skip_interpret_calls () { } } ;
};
}
