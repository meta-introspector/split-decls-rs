// Generated macro for impl_2679 (impl)
macro_rules! Depcrate_features_gen_GpuCommandBufferDescriptorimpl_2679 {
() => {
// Module: crate::features::gen_GpuCommandBufferDescriptor
// Provides: {"impl_2679"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuCommandBufferDescriptor { # [doc = "Construct a new `GpuCommandBufferDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuCommandBufferDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } }
};
}
