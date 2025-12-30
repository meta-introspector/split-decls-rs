// Generated macro for impl_6168 (impl)
macro_rules! Depcrate_features_gen_RtcIdentityProviderimpl_6168 {
() => {
// Module: crate::features::gen_RtcIdentityProvider
// Provides: {"impl_6168"}
// Dependencies: {}
impl RtcIdentityProvider { # [doc = "Construct a new `RtcIdentityProvider`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"] pub fn new (generate_assertion : & :: js_sys :: Function , validate_assertion : & :: js_sys :: Function ,) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_generate_assertion (generate_assertion) ; ret . set_validate_assertion (validate_assertion) ; ret } # [deprecated = "Use `set_generate_assertion()` instead."] pub fn generate_assertion (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_generate_assertion (val) ; self } # [deprecated = "Use `set_validate_assertion()` instead."] pub fn validate_assertion (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_validate_assertion (val) ; self } }
};
}
