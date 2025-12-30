// Generated macro for impl_3993 (impl)
macro_rules! Depcrate_features_gen_ImageDecodeOptionsimpl_3993 {
() => {
// Module: crate::features::gen_ImageDecodeOptions
// Provides: {"impl_3993"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl ImageDecodeOptions { # [doc = "Construct a new `ImageDecodeOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ImageDecodeOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_complete_frames_only()` instead."] pub fn complete_frames_only (& mut self , val : bool) -> & mut Self { self . set_complete_frames_only (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_frame_index()` instead."] pub fn frame_index (& mut self , val : u32) -> & mut Self { self . set_frame_index (val) ; self } }
};
}
