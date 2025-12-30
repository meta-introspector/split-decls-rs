// Generated macro for impl_4443 (impl)
macro_rules! Depcrate_features_gen_MediaPositionStateimpl_4443 {
() => {
// Module: crate::features::gen_MediaPositionState
// Provides: {"impl_4443"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MediaPositionState { # [doc = "Construct a new `MediaPositionState`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_duration()` instead."] pub fn duration (& mut self , val : f64) -> & mut Self { self . set_duration (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_playback_rate()` instead."] pub fn playback_rate (& mut self , val : f64) -> & mut Self { self . set_playback_rate (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_position()` instead."] pub fn position (& mut self , val : f64) -> & mut Self { self . set_position (val) ; self } }
};
}
