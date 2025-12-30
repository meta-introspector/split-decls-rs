// Generated macro for impl_427 (impl)
macro_rules! Depcrate_features_gen_AuthenticationExtensionsDevicePublicKeyInputsimpl_427 {
() => {
// Module: crate::features::gen_AuthenticationExtensionsDevicePublicKeyInputs
// Provides: {"impl_427"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AuthenticationExtensionsDevicePublicKeyInputs { # [doc = "Construct a new `AuthenticationExtensionsDevicePublicKeyInputs`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsDevicePublicKeyInputs`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_attestation()` instead."] pub fn attestation (& mut self , val : & str) -> & mut Self { self . set_attestation (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_attestation_formats()` instead."] pub fn attestation_formats (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_attestation_formats (val) ; self } }
};
}
