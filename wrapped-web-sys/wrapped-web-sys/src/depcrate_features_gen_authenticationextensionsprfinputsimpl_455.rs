// Generated macro for impl_455 (impl)
macro_rules! Depcrate_features_gen_AuthenticationExtensionsPrfInputsimpl_455 {
() => {
// Module: crate::features::gen_AuthenticationExtensionsPrfInputs
// Provides: {"impl_455"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AuthenticationExtensionsPrfInputs { # [doc = "Construct a new `AuthenticationExtensionsPrfInputs`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfInputs`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AuthenticationExtensionsPrfValues")] # [deprecated = "Use `set_eval()` instead."] pub fn eval (& mut self , val : & AuthenticationExtensionsPrfValues) -> & mut Self { self . set_eval (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_eval_by_credential()` instead."] pub fn eval_by_credential (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_eval_by_credential (val) ; self } }
};
}
