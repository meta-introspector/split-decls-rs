// Generated macro for impl_5619 (impl)
macro_rules! Depcrate_features_gen_PublicKeyCredentialRpEntityimpl_5619 {
() => {
// Module: crate::features::gen_PublicKeyCredentialRpEntity
// Provides: {"impl_5619"}
// Dependencies: {}
impl PublicKeyCredentialRpEntity { # [doc = "Construct a new `PublicKeyCredentialRpEntity`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"] pub fn new (name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [deprecated = "Use `set_icon()` instead."] pub fn icon (& mut self , val : & str) -> & mut Self { self . set_icon (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } }
};
}
