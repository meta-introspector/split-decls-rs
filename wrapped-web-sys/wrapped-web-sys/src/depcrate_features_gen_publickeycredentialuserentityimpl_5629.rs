// Generated macro for impl_5629 (impl)
macro_rules! Depcrate_features_gen_PublicKeyCredentialUserEntityimpl_5629 {
() => {
// Module: crate::features::gen_PublicKeyCredentialUserEntity
// Provides: {"impl_5629"}
// Dependencies: {}
impl PublicKeyCredentialUserEntity { # [doc = "Construct a new `PublicKeyCredentialUserEntity`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"] pub fn new (name : & str , display_name : & str , id : & :: js_sys :: Object) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_display_name (display_name) ; ret . set_id (id) ; ret } # [deprecated = "Use `set_icon()` instead."] pub fn icon (& mut self , val : & str) -> & mut Self { self . set_icon (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_display_name()` instead."] pub fn display_name (& mut self , val : & str) -> & mut Self { self . set_display_name (val) ; self } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_id (val) ; self } }
};
}
