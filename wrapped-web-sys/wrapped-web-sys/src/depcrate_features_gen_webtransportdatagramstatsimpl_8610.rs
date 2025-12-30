// Generated macro for impl_8610 (impl)
macro_rules! Depcrate_features_gen_WebTransportDatagramStatsimpl_8610 {
() => {
// Module: crate::features::gen_WebTransportDatagramStats
// Provides: {"impl_8610"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl WebTransportDatagramStats { # [doc = "Construct a new `WebTransportDatagramStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_dropped_incoming()` instead."] pub fn dropped_incoming (& mut self , val : f64) -> & mut Self { self . set_dropped_incoming (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_expired_outgoing()` instead."] pub fn expired_outgoing (& mut self , val : f64) -> & mut Self { self . set_expired_outgoing (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_lost_outgoing()` instead."] pub fn lost_outgoing (& mut self , val : f64) -> & mut Self { self . set_lost_outgoing (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } }
};
}
