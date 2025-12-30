// Generated macro for impl_2626 (impl)
macro_rules! Depcrate_features_gen_GpuBufferDescriptorimpl_2626 {
() => {
// Module: crate::features::gen_GpuBufferDescriptor
// Provides: {"impl_2626"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuBufferDescriptor { # [doc = "Construct a new `GpuBufferDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (size : f64 , usage : u32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_size (size) ; ret . set_usage (usage) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_mapped_at_creation()` instead."] pub fn mapped_at_creation (& mut self , val : bool) -> & mut Self { self . set_mapped_at_creation (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_size()` instead."] pub fn size (& mut self , val : f64) -> & mut Self { self . set_size (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_usage()` instead."] pub fn usage (& mut self , val : u32) -> & mut Self { self . set_usage (val) ; self } }
};
}
