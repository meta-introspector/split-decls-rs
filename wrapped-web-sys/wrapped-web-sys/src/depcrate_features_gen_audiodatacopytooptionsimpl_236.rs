// Generated macro for impl_236 (impl)
macro_rules! Depcrate_features_gen_AudioDataCopyToOptionsimpl_236 {
() => {
// Module: crate::features::gen_AudioDataCopyToOptions
// Provides: {"impl_236"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AudioDataCopyToOptions { # [doc = "Construct a new `AudioDataCopyToOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (plane_index : u32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_plane_index (plane_index) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AudioSampleFormat")] # [deprecated = "Use `set_format()` instead."] pub fn format (& mut self , val : AudioSampleFormat) -> & mut Self { self . set_format (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_frame_count()` instead."] pub fn frame_count (& mut self , val : u32) -> & mut Self { self . set_frame_count (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_frame_offset()` instead."] pub fn frame_offset (& mut self , val : u32) -> & mut Self { self . set_frame_offset (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_plane_index()` instead."] pub fn plane_index (& mut self , val : u32) -> & mut Self { self . set_plane_index (val) ; self } }
};
}
