// Generated macro for impl_2947 (impl)
macro_rules! Depcrate_features_gen_GpuQuerySetDescriptorimpl_2947 {
() => {
// Module: crate::features::gen_GpuQuerySetDescriptor
// Provides: {"impl_2947"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuQuerySetDescriptor { # [cfg (feature = "GpuQueryType")] # [doc = "Construct a new `GpuQuerySetDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`, `GpuQueryType`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (count : u32 , type_ : GpuQueryType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_count (count) ; ret . set_type (type_) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_count()` instead."] pub fn count (& mut self , val : u32) -> & mut Self { self . set_count (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuQueryType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : GpuQueryType) -> & mut Self { self . set_type (val) ; self } }
};
}
