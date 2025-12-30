// Generated macro for impl_5601 (impl)
macro_rules! Depcrate_features_gen_PublicKeyCredentialParametersimpl_5601 {
() => {
// Module: crate::features::gen_PublicKeyCredentialParameters
// Provides: {"impl_5601"}
// Dependencies: {}
impl PublicKeyCredentialParameters { # [cfg (feature = "PublicKeyCredentialType")] # [doc = "Construct a new `PublicKeyCredentialParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`, `PublicKeyCredentialType`*"] pub fn new (alg : i32 , type_ : PublicKeyCredentialType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_alg (alg) ; ret . set_type (type_) ; ret } # [deprecated = "Use `set_alg()` instead."] pub fn alg (& mut self , val : i32) -> & mut Self { self . set_alg (val) ; self } # [cfg (feature = "PublicKeyCredentialType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : PublicKeyCredentialType) -> & mut Self { self . set_type (val) ; self } }
};
}
