// Generated macro for impl_2772 (impl)
macro_rules! Depcrate_features_gen_GpuDeviceDescriptorimpl_2772 {
() => {
// Module: crate::features::gen_GpuDeviceDescriptor
// Provides: {"impl_2772"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuDeviceDescriptor { # [doc = "Construct a new `GpuDeviceDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuDeviceDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuQueueDescriptor")] # [deprecated = "Use `set_default_queue()` instead."] pub fn default_queue (& mut self , val : & GpuQueueDescriptor) -> & mut Self { self . set_default_queue (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_required_features()` instead."] pub fn required_features (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_required_features (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_required_limits()` instead."] pub fn required_limits (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_required_limits (val) ; self } }
};
}
