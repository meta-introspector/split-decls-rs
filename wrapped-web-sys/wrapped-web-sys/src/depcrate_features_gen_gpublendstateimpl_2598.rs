// Generated macro for impl_2598 (impl)
macro_rules! Depcrate_features_gen_GpuBlendStateimpl_2598 {
() => {
// Module: crate::features::gen_GpuBlendState
// Provides: {"impl_2598"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuBlendState { # [cfg (feature = "GpuBlendComponent")] # [doc = "Construct a new `GpuBlendState`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuBlendComponent`, `GpuBlendState`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (alpha : & GpuBlendComponent , color : & GpuBlendComponent) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_alpha (alpha) ; ret . set_color (color) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuBlendComponent")] # [deprecated = "Use `set_alpha()` instead."] pub fn alpha (& mut self , val : & GpuBlendComponent) -> & mut Self { self . set_alpha (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuBlendComponent")] # [deprecated = "Use `set_color()` instead."] pub fn color (& mut self , val : & GpuBlendComponent) -> & mut Self { self . set_color (val) ; self } }
};
}
