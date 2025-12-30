// Generated macro for impl_3126 (impl)
macro_rules! Depcrate_features_gen_GpuTexelCopyBufferLayoutimpl_3126 {
() => {
// Module: crate::features::gen_GpuTexelCopyBufferLayout
// Provides: {"impl_3126"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuTexelCopyBufferLayout { # [doc = "Construct a new `GpuTexelCopyBufferLayout`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuTexelCopyBufferLayout`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bytes_per_row()` instead."] pub fn bytes_per_row (& mut self , val : u32) -> & mut Self { self . set_bytes_per_row (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_offset()` instead."] pub fn offset (& mut self , val : f64) -> & mut Self { self . set_offset (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_rows_per_image()` instead."] pub fn rows_per_image (& mut self , val : u32) -> & mut Self { self . set_rows_per_image (val) ; self } }
};
}
