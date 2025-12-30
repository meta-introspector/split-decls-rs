// Generated macro for impl_2571 (impl)
macro_rules! Depcrate_features_gen_GpuBindGroupLayoutDescriptorimpl_2571 {
() => {
// Module: crate::features::gen_GpuBindGroupLayoutDescriptor
// Provides: {"impl_2571"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuBindGroupLayoutDescriptor { # [doc = "Construct a new `GpuBindGroupLayoutDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (entries : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_entries (entries) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_entries()` instead."] pub fn entries (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_entries (val) ; self } }
};
}
