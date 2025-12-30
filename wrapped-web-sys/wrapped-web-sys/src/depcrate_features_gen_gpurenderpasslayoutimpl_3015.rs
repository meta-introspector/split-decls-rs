// Generated macro for impl_3015 (impl)
macro_rules! Depcrate_features_gen_GpuRenderPassLayoutimpl_3015 {
() => {
// Module: crate::features::gen_GpuRenderPassLayout
// Provides: {"impl_3015"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuRenderPassLayout { # [doc = "Construct a new `GpuRenderPassLayout`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (color_formats : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_color_formats (color_formats) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_color_formats()` instead."] pub fn color_formats (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_color_formats (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuTextureFormat")] # [deprecated = "Use `set_depth_stencil_format()` instead."] pub fn depth_stencil_format (& mut self , val : GpuTextureFormat) -> & mut Self { self . set_depth_stencil_format (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_sample_count()` instead."] pub fn sample_count (& mut self , val : u32) -> & mut Self { self . set_sample_count (val) ; self } }
};
}
