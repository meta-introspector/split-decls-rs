// Generated macro for impl_4669 (impl)
macro_rules! Depcrate_features_gen_MemoryMeasurementimpl_4669 {
() => {
// Module: crate::features::gen_MemoryMeasurement
// Provides: {"impl_4669"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MemoryMeasurement { # [doc = "Construct a new `MemoryMeasurement`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_breakdown()` instead."] pub fn breakdown (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_breakdown (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bytes()` instead."] pub fn bytes (& mut self , val : f64) -> & mut Self { self . set_bytes (val) ; self } }
};
}
