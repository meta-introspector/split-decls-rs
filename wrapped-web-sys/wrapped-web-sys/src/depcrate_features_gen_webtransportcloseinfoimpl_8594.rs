// Generated macro for impl_8594 (impl)
macro_rules! Depcrate_features_gen_WebTransportCloseInfoimpl_8594 {
() => {
// Module: crate::features::gen_WebTransportCloseInfo
// Provides: {"impl_8594"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl WebTransportCloseInfo { # [doc = "Construct a new `WebTransportCloseInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebTransportCloseInfo`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_close_code()` instead."] pub fn close_code (& mut self , val : u32) -> & mut Self { self . set_close_code (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_reason()` instead."] pub fn reason (& mut self , val : & str) -> & mut Self { self . set_reason (val) ; self } }
};
}
