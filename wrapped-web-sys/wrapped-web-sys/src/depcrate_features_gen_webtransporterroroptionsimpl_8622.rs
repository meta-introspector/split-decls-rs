// Generated macro for impl_8622 (impl)
macro_rules! Depcrate_features_gen_WebTransportErrorOptionsimpl_8622 {
() => {
// Module: crate::features::gen_WebTransportErrorOptions
// Provides: {"impl_8622"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl WebTransportErrorOptions { # [doc = "Construct a new `WebTransportErrorOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "WebTransportErrorSource")] # [deprecated = "Use `set_source()` instead."] pub fn source (& mut self , val : WebTransportErrorSource) -> & mut Self { self . set_source (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_stream_error_code()` instead."] pub fn stream_error_code (& mut self , val : Option < u8 >) -> & mut Self { self . set_stream_error_code (val) ; self } }
};
}
