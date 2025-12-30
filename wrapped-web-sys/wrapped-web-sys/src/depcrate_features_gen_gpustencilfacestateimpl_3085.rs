// Generated macro for impl_3085 (impl)
macro_rules! Depcrate_features_gen_GpuStencilFaceStateimpl_3085 {
() => {
// Module: crate::features::gen_GpuStencilFaceState
// Provides: {"impl_3085"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuStencilFaceState { # [doc = "Construct a new `GpuStencilFaceState`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuStencilFaceState`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuCompareFunction")] # [deprecated = "Use `set_compare()` instead."] pub fn compare (& mut self , val : GpuCompareFunction) -> & mut Self { self . set_compare (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuStencilOperation")] # [deprecated = "Use `set_depth_fail_op()` instead."] pub fn depth_fail_op (& mut self , val : GpuStencilOperation) -> & mut Self { self . set_depth_fail_op (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuStencilOperation")] # [deprecated = "Use `set_fail_op()` instead."] pub fn fail_op (& mut self , val : GpuStencilOperation) -> & mut Self { self . set_fail_op (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuStencilOperation")] # [deprecated = "Use `set_pass_op()` instead."] pub fn pass_op (& mut self , val : GpuStencilOperation) -> & mut Self { self . set_pass_op (val) ; self } }
};
}
