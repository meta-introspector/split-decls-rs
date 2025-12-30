// Generated macro for impl_8652 (impl)
macro_rules! Depcrate_features_gen_WebTransportReceiveStreamStatsimpl_8652 {
() => {
// Module: crate::features::gen_WebTransportReceiveStreamStats
// Provides: {"impl_8652"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl WebTransportReceiveStreamStats { # [doc = "Construct a new `WebTransportReceiveStreamStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStreamStats`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bytes_read()` instead."] pub fn bytes_read (& mut self , val : f64) -> & mut Self { self . set_bytes_read (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bytes_received()` instead."] pub fn bytes_received (& mut self , val : f64) -> & mut Self { self . set_bytes_received (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } }
};
}
