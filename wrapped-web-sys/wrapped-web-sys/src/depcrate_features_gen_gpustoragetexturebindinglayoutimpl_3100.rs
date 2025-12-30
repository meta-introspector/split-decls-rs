// Generated macro for impl_3100 (impl)
macro_rules! Depcrate_features_gen_GpuStorageTextureBindingLayoutimpl_3100 {
() => {
// Module: crate::features::gen_GpuStorageTextureBindingLayout
// Provides: {"impl_3100"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuStorageTextureBindingLayout { # [cfg (feature = "GpuTextureFormat")] # [doc = "Construct a new `GpuStorageTextureBindingLayout`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuStorageTextureBindingLayout`, `GpuTextureFormat`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (format : GpuTextureFormat) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_format (format) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuStorageTextureAccess")] # [deprecated = "Use `set_access()` instead."] pub fn access (& mut self , val : GpuStorageTextureAccess) -> & mut Self { self . set_access (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuTextureFormat")] # [deprecated = "Use `set_format()` instead."] pub fn format (& mut self , val : GpuTextureFormat) -> & mut Self { self . set_format (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuTextureViewDimension")] # [deprecated = "Use `set_view_dimension()` instead."] pub fn view_dimension (& mut self , val : GpuTextureViewDimension) -> & mut Self { self . set_view_dimension (val) ; self } }
};
}
