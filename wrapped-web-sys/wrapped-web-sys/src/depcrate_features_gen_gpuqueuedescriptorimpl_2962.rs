// Generated macro for impl_2962 (impl)
macro_rules! Depcrate_features_gen_GpuQueueDescriptorimpl_2962 {
() => {
// Module: crate::features::gen_GpuQueueDescriptor
// Provides: {"impl_2962"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuQueueDescriptor { # [doc = "Construct a new `GpuQueueDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuQueueDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } }
};
}
