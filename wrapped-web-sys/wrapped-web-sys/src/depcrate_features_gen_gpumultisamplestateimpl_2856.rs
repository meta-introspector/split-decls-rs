// Generated macro for impl_2856 (impl)
macro_rules! Depcrate_features_gen_GpuMultisampleStateimpl_2856 {
() => {
// Module: crate::features::gen_GpuMultisampleState
// Provides: {"impl_2856"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuMultisampleState { # [doc = "Construct a new `GpuMultisampleState`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_alpha_to_coverage_enabled()` instead."] pub fn alpha_to_coverage_enabled (& mut self , val : bool) -> & mut Self { self . set_alpha_to_coverage_enabled (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_count()` instead."] pub fn count (& mut self , val : u32) -> & mut Self { self . set_count (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_mask()` instead."] pub fn mask (& mut self , val : u32) -> & mut Self { self . set_mask (val) ; self } }
};
}
