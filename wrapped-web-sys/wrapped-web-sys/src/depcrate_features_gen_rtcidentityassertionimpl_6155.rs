// Generated macro for impl_6155 (impl)
macro_rules! Depcrate_features_gen_RtcIdentityAssertionimpl_6155 {
() => {
// Module: crate::features::gen_RtcIdentityAssertion
// Provides: {"impl_6155"}
// Dependencies: {}
impl RtcIdentityAssertion { # [doc = "Construct a new `RtcIdentityAssertion`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_idp()` instead."] pub fn idp (& mut self , val : & str) -> & mut Self { self . set_idp (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
