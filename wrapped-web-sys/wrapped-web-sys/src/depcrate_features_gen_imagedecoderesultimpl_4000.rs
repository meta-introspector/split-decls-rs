// Generated macro for impl_4000 (impl)
macro_rules! Depcrate_features_gen_ImageDecodeResultimpl_4000 {
() => {
// Module: crate::features::gen_ImageDecodeResult
// Provides: {"impl_4000"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl ImageDecodeResult { # [cfg (feature = "VideoFrame")] # [doc = "Construct a new `ImageDecodeResult`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`, `VideoFrame`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (complete : bool , image : & VideoFrame) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_complete (complete) ; ret . set_image (image) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_complete()` instead."] pub fn complete (& mut self , val : bool) -> & mut Self { self . set_complete (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "VideoFrame")] # [deprecated = "Use `set_image()` instead."] pub fn image (& mut self , val : & VideoFrame) -> & mut Self { self . set_image (val) ; self } }
};
}
