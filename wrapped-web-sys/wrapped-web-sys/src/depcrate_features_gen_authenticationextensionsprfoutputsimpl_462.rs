// Generated macro for impl_462 (impl)
macro_rules! Depcrate_features_gen_AuthenticationExtensionsPrfOutputsimpl_462 {
() => {
// Module: crate::features::gen_AuthenticationExtensionsPrfOutputs
// Provides: {"impl_462"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AuthenticationExtensionsPrfOutputs { # [doc = "Construct a new `AuthenticationExtensionsPrfOutputs`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_enabled()` instead."] pub fn enabled (& mut self , val : bool) -> & mut Self { self . set_enabled (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AuthenticationExtensionsPrfValues")] # [deprecated = "Use `set_results()` instead."] pub fn results (& mut self , val : & AuthenticationExtensionsPrfValues) -> & mut Self { self . set_results (val) ; self } }
};
}
