// Generated macro for impl_2583 (impl)
macro_rules! Depcrate_features_gen_GpuBlendComponentimpl_2583 {
() => {
// Module: crate::features::gen_GpuBlendComponent
// Provides: {"impl_2583"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuBlendComponent { # [doc = "Construct a new `GpuBlendComponent`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuBlendFactor")] # [deprecated = "Use `set_dst_factor()` instead."] pub fn dst_factor (& mut self , val : GpuBlendFactor) -> & mut Self { self . set_dst_factor (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuBlendOperation")] # [deprecated = "Use `set_operation()` instead."] pub fn operation (& mut self , val : GpuBlendOperation) -> & mut Self { self . set_operation (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuBlendFactor")] # [deprecated = "Use `set_src_factor()` instead."] pub fn src_factor (& mut self , val : GpuBlendFactor) -> & mut Self { self . set_src_factor (val) ; self } }
};
}
