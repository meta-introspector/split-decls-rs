// Generated macro for impl_5591 (impl)
macro_rules! Depcrate_features_gen_PublicKeyCredentialEntityimpl_5591 {
() => {
// Module: crate::features::gen_PublicKeyCredentialEntity
// Provides: {"impl_5591"}
// Dependencies: {}
impl PublicKeyCredentialEntity { # [doc = "Construct a new `PublicKeyCredentialEntity`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"] pub fn new (name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [deprecated = "Use `set_icon()` instead."] pub fn icon (& mut self , val : & str) -> & mut Self { self . set_icon (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
