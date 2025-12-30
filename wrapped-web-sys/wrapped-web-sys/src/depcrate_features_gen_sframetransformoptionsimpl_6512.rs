// Generated macro for impl_6512 (impl)
macro_rules! Depcrate_features_gen_SFrameTransformOptionsimpl_6512 {
() => {
// Module: crate::features::gen_SFrameTransformOptions
// Provides: {"impl_6512"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl SFrameTransformOptions { # [doc = "Construct a new `SFrameTransformOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "SFrameTransformRole")] # [deprecated = "Use `set_role()` instead."] pub fn role (& mut self , val : SFrameTransformRole) -> & mut Self { self . set_role (val) ; self } }
};
}
