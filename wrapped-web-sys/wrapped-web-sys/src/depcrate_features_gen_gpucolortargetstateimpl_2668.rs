// Generated macro for impl_2668 (impl)
macro_rules! Depcrate_features_gen_GpuColorTargetStateimpl_2668 {
() => {
// Module: crate::features::gen_GpuColorTargetState
// Provides: {"impl_2668"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuColorTargetState { # [cfg (feature = "GpuTextureFormat")] # [doc = "Construct a new `GpuColorTargetState`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`, `GpuTextureFormat`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (format : GpuTextureFormat) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_format (format) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuBlendState")] # [deprecated = "Use `set_blend()` instead."] pub fn blend (& mut self , val : & GpuBlendState) -> & mut Self { self . set_blend (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuTextureFormat")] # [deprecated = "Use `set_format()` instead."] pub fn format (& mut self , val : GpuTextureFormat) -> & mut Self { self . set_format (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_write_mask()` instead."] pub fn write_mask (& mut self , val : u32) -> & mut Self { self . set_write_mask (val) ; self } }
};
}
