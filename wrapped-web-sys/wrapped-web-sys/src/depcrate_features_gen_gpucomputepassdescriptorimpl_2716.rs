// Generated macro for impl_2716 (impl)
macro_rules! Depcrate_features_gen_GpuComputePassDescriptorimpl_2716 {
() => {
// Module: crate::features::gen_GpuComputePassDescriptor
// Provides: {"impl_2716"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuComputePassDescriptor { # [doc = "Construct a new `GpuComputePassDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuComputePassDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuComputePassTimestampWrites")] # [deprecated = "Use `set_timestamp_writes()` instead."] pub fn timestamp_writes (& mut self , val : & GpuComputePassTimestampWrites) -> & mut Self { self . set_timestamp_writes (val) ; self } }
};
}
