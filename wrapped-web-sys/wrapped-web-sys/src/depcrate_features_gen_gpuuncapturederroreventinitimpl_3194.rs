// Generated macro for impl_3194 (impl)
macro_rules! Depcrate_features_gen_GpuUncapturedErrorEventInitimpl_3194 {
() => {
// Module: crate::features::gen_GpuUncapturedErrorEventInit
// Provides: {"impl_3194"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuUncapturedErrorEventInit { # [cfg (feature = "GpuError")] # [doc = "Construct a new `GpuUncapturedErrorEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuError`, `GpuUncapturedErrorEventInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (error : & GpuError) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_error (error) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuError")] # [deprecated = "Use `set_error()` instead."] pub fn error (& mut self , val : & GpuError) -> & mut Self { self . set_error (val) ; self } }
};
}
