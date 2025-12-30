// Generated macro for impl_469 (impl)
macro_rules! Depcrate_features_gen_AuthenticationExtensionsPrfValuesimpl_469 {
() => {
// Module: crate::features::gen_AuthenticationExtensionsPrfValues
// Provides: {"impl_469"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AuthenticationExtensionsPrfValues { # [doc = "Construct a new `AuthenticationExtensionsPrfValues`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (first : & :: js_sys :: Object) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_first (first) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_first()` instead."] pub fn first (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_first (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_second()` instead."] pub fn second (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_second (val) ; self } }
};
}
