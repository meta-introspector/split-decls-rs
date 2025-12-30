// Generated macro for impl_2728 (impl)
macro_rules! Depcrate_features_gen_GpuComputePassTimestampWritesimpl_2728 {
() => {
// Module: crate::features::gen_GpuComputePassTimestampWrites
// Provides: {"impl_2728"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuComputePassTimestampWrites { # [cfg (feature = "GpuQuerySet")] # [doc = "Construct a new `GpuComputePassTimestampWrites`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuComputePassTimestampWrites`, `GpuQuerySet`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (query_set : & GpuQuerySet) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_query_set (query_set) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_beginning_of_pass_write_index()` instead."] pub fn beginning_of_pass_write_index (& mut self , val : u32) -> & mut Self { self . set_beginning_of_pass_write_index (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_end_of_pass_write_index()` instead."] pub fn end_of_pass_write_index (& mut self , val : u32) -> & mut Self { self . set_end_of_pass_write_index (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuQuerySet")] # [deprecated = "Use `set_query_set()` instead."] pub fn query_set (& mut self , val : & GpuQuerySet) -> & mut Self { self . set_query_set (val) ; self } }
};
}
