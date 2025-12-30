// Generated macro for impl_3211 (impl)
macro_rules! Depcrate_features_gen_GpuVertexBufferLayoutimpl_3211 {
() => {
// Module: crate::features::gen_GpuVertexBufferLayout
// Provides: {"impl_3211"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuVertexBufferLayout { # [doc = "Construct a new `GpuVertexBufferLayout`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (array_stride : f64 , attributes : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_array_stride (array_stride) ; ret . set_attributes (attributes) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_array_stride()` instead."] pub fn array_stride (& mut self , val : f64) -> & mut Self { self . set_array_stride (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_attributes()` instead."] pub fn attributes (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_attributes (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuVertexStepMode")] # [deprecated = "Use `set_step_mode()` instead."] pub fn step_mode (& mut self , val : GpuVertexStepMode) -> & mut Self { self . set_step_mode (val) ; self } }
};
}
