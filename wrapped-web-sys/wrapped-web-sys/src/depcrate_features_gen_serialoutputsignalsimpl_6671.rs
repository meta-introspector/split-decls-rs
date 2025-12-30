// Generated macro for impl_6671 (impl)
macro_rules! Depcrate_features_gen_SerialOutputSignalsimpl_6671 {
() => {
// Module: crate::features::gen_SerialOutputSignals
// Provides: {"impl_6671"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl SerialOutputSignals { # [doc = "Construct a new `SerialOutputSignals`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_break()` instead."] pub fn break_ (& mut self , val : bool) -> & mut Self { self . set_break (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_data_terminal_ready()` instead."] pub fn data_terminal_ready (& mut self , val : bool) -> & mut Self { self . set_data_terminal_ready (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_request_to_send()` instead."] pub fn request_to_send (& mut self , val : bool) -> & mut Self { self . set_request_to_send (val) ; self } }
};
}
