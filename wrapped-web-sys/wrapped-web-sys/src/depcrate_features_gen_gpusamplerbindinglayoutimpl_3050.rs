// Generated macro for impl_3050 (impl)
macro_rules! Depcrate_features_gen_GpuSamplerBindingLayoutimpl_3050 {
() => {
// Module: crate::features::gen_GpuSamplerBindingLayout
// Provides: {"impl_3050"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuSamplerBindingLayout { # [doc = "Construct a new `GpuSamplerBindingLayout`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuSamplerBindingLayout`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuSamplerBindingType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : GpuSamplerBindingType) -> & mut Self { self . set_type (val) ; self } }
};
}
