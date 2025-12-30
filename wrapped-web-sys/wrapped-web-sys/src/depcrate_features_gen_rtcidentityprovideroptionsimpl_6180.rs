// Generated macro for impl_6180 (impl)
macro_rules! Depcrate_features_gen_RtcIdentityProviderOptionsimpl_6180 {
() => {
// Module: crate::features::gen_RtcIdentityProviderOptions
// Provides: {"impl_6180"}
// Dependencies: {}
impl RtcIdentityProviderOptions { # [doc = "Construct a new `RtcIdentityProviderOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_peer_identity()` instead."] pub fn peer_identity (& mut self , val : & str) -> & mut Self { self . set_peer_identity (val) ; self } # [deprecated = "Use `set_protocol()` instead."] pub fn protocol (& mut self , val : & str) -> & mut Self { self . set_protocol (val) ; self } # [deprecated = "Use `set_username_hint()` instead."] pub fn username_hint (& mut self , val : & str) -> & mut Self { self . set_username_hint (val) ; self } }
};
}
