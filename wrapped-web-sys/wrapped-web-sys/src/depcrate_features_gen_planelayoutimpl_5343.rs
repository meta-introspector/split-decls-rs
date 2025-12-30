// Generated macro for impl_5343 (impl)
macro_rules! Depcrate_features_gen_PlaneLayoutimpl_5343 {
() => {
// Module: crate::features::gen_PlaneLayout
// Provides: {"impl_5343"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl PlaneLayout { # [doc = "Construct a new `PlaneLayout`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (offset : u32 , stride : u32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_offset (offset) ; ret . set_stride (stride) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_offset()` instead."] pub fn offset (& mut self , val : u32) -> & mut Self { self . set_offset (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_stride()` instead."] pub fn stride (& mut self , val : u32) -> & mut Self { self . set_stride (val) ; self } }
};
}
