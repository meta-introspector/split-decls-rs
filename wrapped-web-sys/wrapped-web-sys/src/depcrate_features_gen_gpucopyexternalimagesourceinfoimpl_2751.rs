// Generated macro for impl_2751 (impl)
macro_rules! Depcrate_features_gen_GpuCopyExternalImageSourceInfoimpl_2751 {
() => {
// Module: crate::features::gen_GpuCopyExternalImageSourceInfo
// Provides: {"impl_2751"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuCopyExternalImageSourceInfo { # [doc = "Construct a new `GpuCopyExternalImageSourceInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageSourceInfo`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (source : & :: js_sys :: Object) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_source (source) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_flip_y()` instead."] pub fn flip_y (& mut self , val : bool) -> & mut Self { self . set_flip_y (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_origin()` instead."] pub fn origin (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_origin (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_source()` instead."] pub fn source (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_source (val) ; self } }
};
}
