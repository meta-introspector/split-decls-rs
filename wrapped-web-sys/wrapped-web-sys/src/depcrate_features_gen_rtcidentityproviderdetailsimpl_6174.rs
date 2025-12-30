// Generated macro for impl_6174 (impl)
macro_rules! Depcrate_features_gen_RtcIdentityProviderDetailsimpl_6174 {
() => {
// Module: crate::features::gen_RtcIdentityProviderDetails
// Provides: {"impl_6174"}
// Dependencies: {}
impl RtcIdentityProviderDetails { # [doc = "Construct a new `RtcIdentityProviderDetails`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"] pub fn new (domain : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_domain (domain) ; ret } # [deprecated = "Use `set_domain()` instead."] pub fn domain (& mut self , val : & str) -> & mut Self { self . set_domain (val) ; self } # [deprecated = "Use `set_protocol()` instead."] pub fn protocol (& mut self , val : & str) -> & mut Self { self . set_protocol (val) ; self } }
};
}
