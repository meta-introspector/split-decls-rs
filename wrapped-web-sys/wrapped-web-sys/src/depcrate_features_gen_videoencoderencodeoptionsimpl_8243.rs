// Generated macro for impl_8243 (impl)
macro_rules! Depcrate_features_gen_VideoEncoderEncodeOptionsimpl_8243 {
() => {
// Module: crate::features::gen_VideoEncoderEncodeOptions
// Provides: {"impl_8243"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl VideoEncoderEncodeOptions { # [doc = "Construct a new `VideoEncoderEncodeOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_key_frame()` instead."] pub fn key_frame (& mut self , val : bool) -> & mut Self { self . set_key_frame (val) ; self } }
};
}
