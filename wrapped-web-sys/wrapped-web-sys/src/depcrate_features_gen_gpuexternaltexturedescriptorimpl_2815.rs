// Generated macro for impl_2815 (impl)
macro_rules! Depcrate_features_gen_GpuExternalTextureDescriptorimpl_2815 {
() => {
// Module: crate::features::gen_GpuExternalTextureDescriptor
// Provides: {"impl_2815"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuExternalTextureDescriptor { # [doc = "Construct a new `GpuExternalTextureDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuExternalTextureDescriptor`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (source : & :: js_sys :: Object) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_source (source) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_source()` instead."] pub fn source (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_source (val) ; self } }
};
}
