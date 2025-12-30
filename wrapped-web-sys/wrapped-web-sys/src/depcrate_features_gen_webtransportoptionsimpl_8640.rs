// Generated macro for impl_8640 (impl)
macro_rules! Depcrate_features_gen_WebTransportOptionsimpl_8640 {
() => {
// Module: crate::features::gen_WebTransportOptions
// Provides: {"impl_8640"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl WebTransportOptions { # [doc = "Construct a new `WebTransportOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_allow_pooling()` instead."] pub fn allow_pooling (& mut self , val : bool) -> & mut Self { self . set_allow_pooling (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "WebTransportCongestionControl")] # [deprecated = "Use `set_congestion_control()` instead."] pub fn congestion_control (& mut self , val : WebTransportCongestionControl) -> & mut Self { self . set_congestion_control (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_require_unreliable()` instead."] pub fn require_unreliable (& mut self , val : bool) -> & mut Self { self . set_require_unreliable (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_server_certificate_hashes()` instead."] pub fn server_certificate_hashes (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_server_certificate_hashes (val) ; self } }
};
}
