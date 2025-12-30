// Generated macro for impl_8675 (impl)
macro_rules! Depcrate_features_gen_WebTransportSendStreamStatsimpl_8675 {
() => {
// Module: crate::features::gen_WebTransportSendStreamStats
// Provides: {"impl_8675"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl WebTransportSendStreamStats { # [doc = "Construct a new `WebTransportSendStreamStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bytes_acknowledged()` instead."] pub fn bytes_acknowledged (& mut self , val : f64) -> & mut Self { self . set_bytes_acknowledged (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bytes_sent()` instead."] pub fn bytes_sent (& mut self , val : f64) -> & mut Self { self . set_bytes_sent (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bytes_written()` instead."] pub fn bytes_written (& mut self , val : f64) -> & mut Self { self . set_bytes_written (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } }
};
}
