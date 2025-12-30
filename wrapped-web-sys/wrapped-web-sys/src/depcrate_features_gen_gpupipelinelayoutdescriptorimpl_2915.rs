// Generated macro for impl_2915 (impl)
macro_rules! Depcrate_features_gen_GpuPipelineLayoutDescriptorimpl_2915 {
() => {
// Module: crate::features::gen_GpuPipelineLayoutDescriptor
// Provides: {"impl_2915"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuPipelineLayoutDescriptor { # [doc = "Construct a new `GpuPipelineLayoutDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuPipelineLayoutDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (bind_group_layouts : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_bind_group_layouts (bind_group_layouts) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bind_group_layouts()` instead."] pub fn bind_group_layouts (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_bind_group_layouts (val) ; self } }
};
}
