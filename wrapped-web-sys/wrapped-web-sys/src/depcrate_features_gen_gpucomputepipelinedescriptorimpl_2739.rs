// Generated macro for impl_2739 (impl)
macro_rules! Depcrate_features_gen_GpuComputePipelineDescriptorimpl_2739 {
() => {
// Module: crate::features::gen_GpuComputePipelineDescriptor
// Provides: {"impl_2739"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuComputePipelineDescriptor { # [cfg (feature = "GpuProgrammableStage")] # [doc = "Construct a new `GpuComputePipelineDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuComputePipelineDescriptor`, `GpuProgrammableStage`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (layout : & :: wasm_bindgen :: JsValue , compute : & GpuProgrammableStage) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_layout (layout) ; ret . set_compute (compute) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_layout()` instead."] pub fn layout (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_layout (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuProgrammableStage")] # [deprecated = "Use `set_compute()` instead."] pub fn compute (& mut self , val : & GpuProgrammableStage) -> & mut Self { self . set_compute (val) ; self } }
};
}
