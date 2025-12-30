// Generated macro for impl_6082 (impl)
macro_rules! Depcrate_features_gen_RtcEncodedVideoFrameOptionsimpl_6082 {
() => {
// Module: crate::features::gen_RtcEncodedVideoFrameOptions
// Provides: {"impl_6082"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl RtcEncodedVideoFrameOptions { # [doc = "Construct a new `RtcEncodedVideoFrameOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "RtcEncodedVideoFrameMetadata")] # [deprecated = "Use `set_metadata()` instead."] pub fn metadata (& mut self , val : & RtcEncodedVideoFrameMetadata) -> & mut Self { self . set_metadata (val) ; self } }
};
}
