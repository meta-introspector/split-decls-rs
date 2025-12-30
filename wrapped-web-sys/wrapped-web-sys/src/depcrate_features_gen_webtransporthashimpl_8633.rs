// Generated macro for impl_8633 (impl)
macro_rules! Depcrate_features_gen_WebTransportHashimpl_8633 {
() => {
// Module: crate::features::gen_WebTransportHash
// Provides: {"impl_8633"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl WebTransportHash { # [doc = "Construct a new `WebTransportHash`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebTransportHash`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_algorithm()` instead."] pub fn algorithm (& mut self , val : & str) -> & mut Self { self . set_algorithm (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_value (val) ; self } }
};
}
