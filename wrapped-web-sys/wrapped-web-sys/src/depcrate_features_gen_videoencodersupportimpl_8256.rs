// Generated macro for impl_8256 (impl)
macro_rules! Depcrate_features_gen_VideoEncoderSupportimpl_8256 {
() => {
// Module: crate::features::gen_VideoEncoderSupport
// Provides: {"impl_8256"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl VideoEncoderSupport { # [doc = "Construct a new `VideoEncoderSupport`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `VideoEncoderSupport`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "VideoEncoderConfig")] # [deprecated = "Use `set_config()` instead."] pub fn config (& mut self , val : & VideoEncoderConfig) -> & mut Self { self . set_config (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_supported()` instead."] pub fn supported (& mut self , val : bool) -> & mut Self { self . set_supported (val) ; self } }
};
}
