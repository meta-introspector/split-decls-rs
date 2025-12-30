// Generated macro for impl_3079 (impl)
macro_rules! Depcrate_features_gen_GpuShaderModuleDescriptorimpl_3079 {
() => {
// Module: crate::features::gen_GpuShaderModuleDescriptor
// Provides: {"impl_3079"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuShaderModuleDescriptor { # [doc = "Construct a new `GpuShaderModuleDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuShaderModuleDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (code : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_code (code) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_code()` instead."] pub fn code (& mut self , val : & str) -> & mut Self { self . set_code (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_compilation_hints()` instead."] pub fn compilation_hints (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_compilation_hints (val) ; self } }
};
}
