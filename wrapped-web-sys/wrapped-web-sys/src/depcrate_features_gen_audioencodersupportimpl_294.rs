// Generated macro for impl_294 (impl)
macro_rules! Depcrate_features_gen_AudioEncoderSupportimpl_294 {
() => {
// Module: crate::features::gen_AudioEncoderSupport
// Provides: {"impl_294"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AudioEncoderSupport { # [doc = "Construct a new `AudioEncoderSupport`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AudioEncoderConfig")] # [deprecated = "Use `set_config()` instead."] pub fn config (& mut self , val : & AudioEncoderConfig) -> & mut Self { self . set_config (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_supported()` instead."] pub fn supported (& mut self , val : bool) -> & mut Self { self . set_supported (val) ; self } }
};
}
