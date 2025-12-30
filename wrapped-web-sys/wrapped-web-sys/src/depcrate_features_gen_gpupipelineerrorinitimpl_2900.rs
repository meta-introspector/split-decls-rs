// Generated macro for impl_2900 (impl)
macro_rules! Depcrate_features_gen_GpuPipelineErrorInitimpl_2900 {
() => {
// Module: crate::features::gen_GpuPipelineErrorInit
// Provides: {"impl_2900"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuPipelineErrorInit { # [cfg (feature = "GpuPipelineErrorReason")] # [doc = "Construct a new `GpuPipelineErrorInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (reason : GpuPipelineErrorReason) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_reason (reason) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuPipelineErrorReason")] # [deprecated = "Use `set_reason()` instead."] pub fn reason (& mut self , val : GpuPipelineErrorReason) -> & mut Self { self . set_reason (val) ; self } }
};
}
