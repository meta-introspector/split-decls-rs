// Generated macro for impl_265 (impl)
macro_rules! Depcrate_features_gen_AudioDecoderSupportimpl_265 {
() => {
// Module: crate::features::gen_AudioDecoderSupport
// Provides: {"impl_265"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AudioDecoderSupport { # [doc = "Construct a new `AudioDecoderSupport`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioDecoderSupport`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AudioDecoderConfig")] # [deprecated = "Use `set_config()` instead."] pub fn config (& mut self , val : & AudioDecoderConfig) -> & mut Self { self . set_config (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_supported()` instead."] pub fn supported (& mut self , val : bool) -> & mut Self { self . set_supported (val) ; self } }
};
}
