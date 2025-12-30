// Generated macro for impl_2889 (impl)
macro_rules! Depcrate_features_gen_GpuPipelineDescriptorBaseimpl_2889 {
() => {
// Module: crate::features::gen_GpuPipelineDescriptorBase
// Provides: {"impl_2889"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuPipelineDescriptorBase { # [doc = "Construct a new `GpuPipelineDescriptorBase`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuPipelineDescriptorBase`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (layout : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_layout (layout) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_layout()` instead."] pub fn layout (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_layout (val) ; self } }
};
}
