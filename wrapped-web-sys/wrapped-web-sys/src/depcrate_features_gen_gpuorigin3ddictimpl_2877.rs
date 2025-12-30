// Generated macro for impl_2877 (impl)
macro_rules! Depcrate_features_gen_GpuOrigin3dDictimpl_2877 {
() => {
// Module: crate::features::gen_GpuOrigin3dDict
// Provides: {"impl_2877"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuOrigin3dDict { # [doc = "Construct a new `GpuOrigin3dDict`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuOrigin3dDict`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_x()` instead."] pub fn x (& mut self , val : u32) -> & mut Self { self . set_x (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_y()` instead."] pub fn y (& mut self , val : u32) -> & mut Self { self . set_y (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_z()` instead."] pub fn z (& mut self , val : u32) -> & mut Self { self . set_z (val) ; self } }
};
}
