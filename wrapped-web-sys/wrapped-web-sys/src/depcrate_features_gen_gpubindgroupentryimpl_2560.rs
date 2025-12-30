// Generated macro for impl_2560 (impl)
macro_rules! Depcrate_features_gen_GpuBindGroupEntryimpl_2560 {
() => {
// Module: crate::features::gen_GpuBindGroupEntry
// Provides: {"impl_2560"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuBindGroupEntry { # [doc = "Construct a new `GpuBindGroupEntry`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuBindGroupEntry`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (binding : u32 , resource : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_binding (binding) ; ret . set_resource (resource) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_binding()` instead."] pub fn binding (& mut self , val : u32) -> & mut Self { self . set_binding (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_resource()` instead."] pub fn resource (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_resource (val) ; self } }
};
}
