// Generated macro for impl_6063 (impl)
macro_rules! Depcrate_features_gen_RtcEncodedAudioFrameOptionsimpl_6063 {
() => {
// Module: crate::features::gen_RtcEncodedAudioFrameOptions
// Provides: {"impl_6063"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl RtcEncodedAudioFrameOptions { # [doc = "Construct a new `RtcEncodedAudioFrameOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "RtcEncodedAudioFrameMetadata")] # [deprecated = "Use `set_metadata()` instead."] pub fn metadata (& mut self , val : & RtcEncodedAudioFrameMetadata) -> & mut Self { self . set_metadata (val) ; self } }
};
}
