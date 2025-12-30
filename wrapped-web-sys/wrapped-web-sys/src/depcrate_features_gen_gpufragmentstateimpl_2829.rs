// Generated macro for impl_2829 (impl)
macro_rules! Depcrate_features_gen_GpuFragmentStateimpl_2829 {
() => {
// Module: crate::features::gen_GpuFragmentState
// Provides: {"impl_2829"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuFragmentState { # [cfg (feature = "GpuShaderModule")] # [doc = "Construct a new `GpuFragmentState`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (module : & GpuShaderModule , targets : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_module (module) ; ret . set_targets (targets) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_constants()` instead."] pub fn constants (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_constants (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_entry_point()` instead."] pub fn entry_point (& mut self , val : & str) -> & mut Self { self . set_entry_point (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuShaderModule")] # [deprecated = "Use `set_module()` instead."] pub fn module (& mut self , val : & GpuShaderModule) -> & mut Self { self . set_module (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_targets()` instead."] pub fn targets (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_targets (val) ; self } }
};
}
