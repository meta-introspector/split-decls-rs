// Generated macro for impl_7947 (impl)
macro_rules! Depcrate_features_gen_ULongRangeimpl_7947 {
() => {
// Module: crate::features::gen_ULongRange
// Provides: {"impl_7947"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl ULongRange { # [doc = "Construct a new `ULongRange`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ULongRange`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_max()` instead."] pub fn max (& mut self , val : u32) -> & mut Self { self . set_max (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_min()` instead."] pub fn min (& mut self , val : u32) -> & mut Self { self . set_min (val) ; self } }
};
}
