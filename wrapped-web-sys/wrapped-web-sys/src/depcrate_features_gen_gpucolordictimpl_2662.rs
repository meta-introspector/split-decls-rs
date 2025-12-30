// Generated macro for impl_2662 (impl)
macro_rules! Depcrate_features_gen_GpuColorDictimpl_2662 {
() => {
// Module: crate::features::gen_GpuColorDict
// Provides: {"impl_2662"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuColorDict { # [doc = "Construct a new `GpuColorDict`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (a : f64 , b : f64 , g : f64 , r : f64) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_a (a) ; ret . set_b (b) ; ret . set_g (g) ; ret . set_r (r) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_a()` instead."] pub fn a (& mut self , val : f64) -> & mut Self { self . set_a (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_b()` instead."] pub fn b (& mut self , val : f64) -> & mut Self { self . set_b (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_g()` instead."] pub fn g (& mut self , val : f64) -> & mut Self { self . set_g (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_r()` instead."] pub fn r (& mut self , val : f64) -> & mut Self { self . set_r (val) ; self } }
};
}
