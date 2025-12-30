// Generated macro for impl_2615 (impl)
macro_rules! Depcrate_features_gen_GpuBufferBindingLayoutimpl_2615 {
() => {
// Module: crate::features::gen_GpuBufferBindingLayout
// Provides: {"impl_2615"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuBufferBindingLayout { # [doc = "Construct a new `GpuBufferBindingLayout`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuBufferBindingLayout`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_has_dynamic_offset()` instead."] pub fn has_dynamic_offset (& mut self , val : bool) -> & mut Self { self . set_has_dynamic_offset (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_min_binding_size()` instead."] pub fn min_binding_size (& mut self , val : f64) -> & mut Self { self . set_min_binding_size (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuBufferBindingType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : GpuBufferBindingType) -> & mut Self { self . set_type (val) ; self } }
};
}
