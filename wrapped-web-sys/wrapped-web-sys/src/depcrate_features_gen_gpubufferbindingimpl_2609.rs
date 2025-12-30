// Generated macro for impl_2609 (impl)
macro_rules! Depcrate_features_gen_GpuBufferBindingimpl_2609 {
() => {
// Module: crate::features::gen_GpuBufferBinding
// Provides: {"impl_2609"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuBufferBinding { # [cfg (feature = "GpuBuffer")] # [doc = "Construct a new `GpuBufferBinding`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferBinding`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (buffer : & GpuBuffer) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_buffer (buffer) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuBuffer")] # [deprecated = "Use `set_buffer()` instead."] pub fn buffer (& mut self , val : & GpuBuffer) -> & mut Self { self . set_buffer (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_offset()` instead."] pub fn offset (& mut self , val : f64) -> & mut Self { self . set_offset (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_size()` instead."] pub fn size (& mut self , val : f64) -> & mut Self { self . set_size (val) ; self } }
};
}
