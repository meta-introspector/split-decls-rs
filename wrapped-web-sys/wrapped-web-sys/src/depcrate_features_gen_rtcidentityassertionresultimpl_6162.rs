// Generated macro for impl_6162 (impl)
macro_rules! Depcrate_features_gen_RtcIdentityAssertionResultimpl_6162 {
() => {
// Module: crate::features::gen_RtcIdentityAssertionResult
// Provides: {"impl_6162"}
// Dependencies: {}
impl RtcIdentityAssertionResult { # [cfg (feature = "RtcIdentityProviderDetails")] # [doc = "Construct a new `RtcIdentityAssertionResult`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"] pub fn new (assertion : & str , idp : & RtcIdentityProviderDetails) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_assertion (assertion) ; ret . set_idp (idp) ; ret } # [deprecated = "Use `set_assertion()` instead."] pub fn assertion (& mut self , val : & str) -> & mut Self { self . set_assertion (val) ; self } # [cfg (feature = "RtcIdentityProviderDetails")] # [deprecated = "Use `set_idp()` instead."] pub fn idp (& mut self , val : & RtcIdentityProviderDetails) -> & mut Self { self . set_idp (val) ; self } }
};
}
