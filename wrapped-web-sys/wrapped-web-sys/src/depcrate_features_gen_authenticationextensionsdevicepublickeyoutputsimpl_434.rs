// Generated macro for impl_434 (impl)
macro_rules! Depcrate_features_gen_AuthenticationExtensionsDevicePublicKeyOutputsimpl_434 {
() => {
// Module: crate::features::gen_AuthenticationExtensionsDevicePublicKeyOutputs
// Provides: {"impl_434"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl AuthenticationExtensionsDevicePublicKeyOutputs { # [doc = "Construct a new `AuthenticationExtensionsDevicePublicKeyOutputs`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsDevicePublicKeyOutputs`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_signature()` instead."] pub fn signature (& mut self , val : & :: js_sys :: ArrayBuffer) -> & mut Self { self . set_signature (val) ; self } }
};
}
