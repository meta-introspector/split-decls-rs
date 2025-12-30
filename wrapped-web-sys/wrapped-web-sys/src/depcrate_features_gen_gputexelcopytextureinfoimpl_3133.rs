// Generated macro for impl_3133 (impl)
macro_rules! Depcrate_features_gen_GpuTexelCopyTextureInfoimpl_3133 {
() => {
// Module: crate::features::gen_GpuTexelCopyTextureInfo
// Provides: {"impl_3133"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuTexelCopyTextureInfo { # [cfg (feature = "GpuTexture")] # [doc = "Construct a new `GpuTexelCopyTextureInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuTexelCopyTextureInfo`, `GpuTexture`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (texture : & GpuTexture) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_texture (texture) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuTextureAspect")] # [deprecated = "Use `set_aspect()` instead."] pub fn aspect (& mut self , val : GpuTextureAspect) -> & mut Self { self . set_aspect (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_mip_level()` instead."] pub fn mip_level (& mut self , val : u32) -> & mut Self { self . set_mip_level (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_origin()` instead."] pub fn origin (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_origin (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuTexture")] # [deprecated = "Use `set_texture()` instead."] pub fn texture (& mut self , val : & GpuTexture) -> & mut Self { self . set_texture (val) ; self } }
};
}
