// Generated macro for impl_4603 (impl)
macro_rules! Depcrate_features_gen_MediaStreamTrackProcessorInitimpl_4603 {
() => {
// Module: crate::features::gen_MediaStreamTrackProcessorInit
// Provides: {"impl_4603"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MediaStreamTrackProcessorInit { # [cfg (feature = "MediaStreamTrack")] # [doc = "Construct a new `MediaStreamTrackProcessorInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackProcessorInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (track : & MediaStreamTrack) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_track (track) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_max_buffer_size()` instead."] pub fn max_buffer_size (& mut self , val : u16) -> & mut Self { self . set_max_buffer_size (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "MediaStreamTrack")] # [deprecated = "Use `set_track()` instead."] pub fn track (& mut self , val : & MediaStreamTrack) -> & mut Self { self . set_track (val) ; self } }
};
}
