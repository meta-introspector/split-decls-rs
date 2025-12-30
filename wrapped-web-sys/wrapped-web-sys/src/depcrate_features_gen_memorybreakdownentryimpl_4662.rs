// Generated macro for impl_4662 (impl)
macro_rules! Depcrate_features_gen_MemoryBreakdownEntryimpl_4662 {
() => {
// Module: crate::features::gen_MemoryBreakdownEntry
// Provides: {"impl_4662"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MemoryBreakdownEntry { # [doc = "Construct a new `MemoryBreakdownEntry`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_attribution()` instead."] pub fn attribution (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_attribution (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bytes()` instead."] pub fn bytes (& mut self , val : f64) -> & mut Self { self . set_bytes (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_types()` instead."] pub fn types (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_types (val) ; self } }
};
}
