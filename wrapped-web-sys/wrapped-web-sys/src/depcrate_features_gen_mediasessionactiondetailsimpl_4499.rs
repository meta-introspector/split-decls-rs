// Generated macro for impl_4499 (impl)
macro_rules! Depcrate_features_gen_MediaSessionActionDetailsimpl_4499 {
() => {
// Module: crate::features::gen_MediaSessionActionDetails
// Provides: {"impl_4499"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MediaSessionActionDetails { # [cfg (feature = "MediaSessionAction")] # [doc = "Construct a new `MediaSessionActionDetails`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (action : MediaSessionAction) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_action (action) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "MediaSessionAction")] # [deprecated = "Use `set_action()` instead."] pub fn action (& mut self , val : MediaSessionAction) -> & mut Self { self . set_action (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_fast_seek()` instead."] pub fn fast_seek (& mut self , val : Option < bool >) -> & mut Self { self . set_fast_seek (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_seek_offset()` instead."] pub fn seek_offset (& mut self , val : Option < f64 >) -> & mut Self { self . set_seek_offset (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_seek_time()` instead."] pub fn seek_time (& mut self , val : Option < f64 >) -> & mut Self { self . set_seek_time (val) ; self } }
};
}
