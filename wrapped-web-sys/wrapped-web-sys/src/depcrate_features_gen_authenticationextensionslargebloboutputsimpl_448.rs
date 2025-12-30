// Generated macro for impl_448 (impl)
macro_rules! Depcrate_features_gen_AuthenticationExtensionsLargeBlobOutputsimpl_448 {
() => {
// Module: crate::features::gen_AuthenticationExtensionsLargeBlobOutputs
// Provides: {"impl_448"}
// Dependencies: {}
impl AuthenticationExtensionsLargeBlobOutputs { # [doc = "Construct a new `AuthenticationExtensionsLargeBlobOutputs`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_blob()` instead."] pub fn blob (& mut self , val : & :: js_sys :: ArrayBuffer) -> & mut Self { self . set_blob (val) ; self } # [deprecated = "Use `set_supported()` instead."] pub fn supported (& mut self , val : bool) -> & mut Self { self . set_supported (val) ; self } # [deprecated = "Use `set_written()` instead."] pub fn written (& mut self , val : bool) -> & mut Self { self . set_written (val) ; self } }
};
}
