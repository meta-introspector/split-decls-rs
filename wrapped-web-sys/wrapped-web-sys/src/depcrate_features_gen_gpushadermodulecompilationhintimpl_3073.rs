// Generated macro for impl_3073 (impl)
macro_rules! Depcrate_features_gen_GpuShaderModuleCompilationHintimpl_3073 {
() => {
// Module: crate::features::gen_GpuShaderModuleCompilationHint
// Provides: {"impl_3073"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuShaderModuleCompilationHint { # [doc = "Construct a new `GpuShaderModuleCompilationHint`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuShaderModuleCompilationHint`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (entry_point : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_entry_point (entry_point) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_entry_point()` instead."] pub fn entry_point (& mut self , val : & str) -> & mut Self { self . set_entry_point (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_layout()` instead."] pub fn layout (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_layout (val) ; self } }
};
}
