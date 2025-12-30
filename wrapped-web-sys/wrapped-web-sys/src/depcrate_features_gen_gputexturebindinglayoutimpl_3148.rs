// Generated macro for impl_3148 (impl)
macro_rules! Depcrate_features_gen_GpuTextureBindingLayoutimpl_3148 {
() => {
// Module: crate::features::gen_GpuTextureBindingLayout
// Provides: {"impl_3148"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuTextureBindingLayout { # [doc = "Construct a new `GpuTextureBindingLayout`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_multisampled()` instead."] pub fn multisampled (& mut self , val : bool) -> & mut Self { self . set_multisampled (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuTextureSampleType")] # [deprecated = "Use `set_sample_type()` instead."] pub fn sample_type (& mut self , val : GpuTextureSampleType) -> & mut Self { self . set_sample_type (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuTextureViewDimension")] # [deprecated = "Use `set_view_dimension()` instead."] pub fn view_dimension (& mut self , val : GpuTextureViewDimension) -> & mut Self { self . set_view_dimension (val) ; self } }
};
}
