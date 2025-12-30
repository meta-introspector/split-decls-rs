// Generated macro for impl_3205 (impl)
macro_rules! Depcrate_features_gen_GpuVertexAttributeimpl_3205 {
() => {
// Module: crate::features::gen_GpuVertexAttribute
// Provides: {"impl_3205"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuVertexAttribute { # [cfg (feature = "GpuVertexFormat")] # [doc = "Construct a new `GpuVertexAttribute`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuVertexAttribute`, `GpuVertexFormat`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (format : GpuVertexFormat , offset : f64 , shader_location : u32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_format (format) ; ret . set_offset (offset) ; ret . set_shader_location (shader_location) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuVertexFormat")] # [deprecated = "Use `set_format()` instead."] pub fn format (& mut self , val : GpuVertexFormat) -> & mut Self { self . set_format (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_offset()` instead."] pub fn offset (& mut self , val : f64) -> & mut Self { self . set_offset (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_shader_location()` instead."] pub fn shader_location (& mut self , val : u32) -> & mut Self { self . set_shader_location (val) ; self } }
};
}
