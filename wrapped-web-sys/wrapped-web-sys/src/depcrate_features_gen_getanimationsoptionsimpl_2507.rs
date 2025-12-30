// Generated macro for impl_2507 (impl)
macro_rules! Depcrate_features_gen_GetAnimationsOptionsimpl_2507 {
() => {
// Module: crate::features::gen_GetAnimationsOptions
// Provides: {"impl_2507"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GetAnimationsOptions { # [doc = "Construct a new `GetAnimationsOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GetAnimationsOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_subtree()` instead."] pub fn subtree (& mut self , val : bool) -> & mut Self { self . set_subtree (val) ; self } }
};
}
