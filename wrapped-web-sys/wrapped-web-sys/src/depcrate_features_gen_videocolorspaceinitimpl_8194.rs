// Generated macro for impl_8194 (impl)
macro_rules! Depcrate_features_gen_VideoColorSpaceInitimpl_8194 {
() => {
// Module: crate::features::gen_VideoColorSpaceInit
// Provides: {"impl_8194"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl VideoColorSpaceInit { # [doc = "Construct a new `VideoColorSpaceInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_full_range()` instead."] pub fn full_range (& mut self , val : bool) -> & mut Self { self . set_full_range (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "VideoMatrixCoefficients")] # [deprecated = "Use `set_matrix()` instead."] pub fn matrix (& mut self , val : VideoMatrixCoefficients) -> & mut Self { self . set_matrix (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "VideoColorPrimaries")] # [deprecated = "Use `set_primaries()` instead."] pub fn primaries (& mut self , val : VideoColorPrimaries) -> & mut Self { self . set_primaries (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "VideoTransferCharacteristics")] # [deprecated = "Use `set_transfer()` instead."] pub fn transfer (& mut self , val : VideoTransferCharacteristics) -> & mut Self { self . set_transfer (val) ; self } }
};
}
