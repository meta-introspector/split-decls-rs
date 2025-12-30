// Generated macro for impl_2797 (impl)
macro_rules! Depcrate_features_gen_GpuExtent3dDictimpl_2797 {
() => {
// Module: crate::features::gen_GpuExtent3dDict
// Provides: {"impl_2797"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuExtent3dDict { # [doc = "Construct a new `GpuExtent3dDict`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (width : u32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_width (width) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_depth_or_array_layers()` instead."] pub fn depth_or_array_layers (& mut self , val : u32) -> & mut Self { self . set_depth_or_array_layers (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_height()` instead."] pub fn height (& mut self , val : u32) -> & mut Self { self . set_height (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_width()` instead."] pub fn width (& mut self , val : u32) -> & mut Self { self . set_width (val) ; self } }
};
}
