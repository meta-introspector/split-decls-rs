// Generated macro for impl_4083 (impl)
macro_rules! Depcrate_features_gen_IsInputPendingOptionsimpl_4083 {
() => {
// Module: crate::features::gen_IsInputPendingOptions
// Provides: {"impl_4083"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl IsInputPendingOptions { # [doc = "Construct a new `IsInputPendingOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_include_continuous()` instead."] pub fn include_continuous (& mut self , val : bool) -> & mut Self { self . set_include_continuous (val) ; self } }
};
}
