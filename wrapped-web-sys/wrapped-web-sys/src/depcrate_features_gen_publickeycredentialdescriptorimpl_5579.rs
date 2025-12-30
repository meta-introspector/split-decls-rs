// Generated macro for impl_5579 (impl)
macro_rules! Depcrate_features_gen_PublicKeyCredentialDescriptorimpl_5579 {
() => {
// Module: crate::features::gen_PublicKeyCredentialDescriptor
// Provides: {"impl_5579"}
// Dependencies: {}
impl PublicKeyCredentialDescriptor { # [cfg (feature = "PublicKeyCredentialType")] # [doc = "Construct a new `PublicKeyCredentialDescriptor`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"] pub fn new (id : & :: js_sys :: Object , type_ : PublicKeyCredentialType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_id (id) ; ret . set_type (type_) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_transports()` instead."] pub fn transports (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_transports (val) ; self } # [cfg (feature = "PublicKeyCredentialType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : PublicKeyCredentialType) -> & mut Self { self . set_type (val) ; self } }
};
}
